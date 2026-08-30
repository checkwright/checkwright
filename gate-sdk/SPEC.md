# gate-sdk — a self-testing lint framework for prose/spec/config surfaces

Machine-gated consistency for the surfaces conventional linters ignore:
markdown specs, glossaries, task queues, config projections, diagrams — any
text whose drift is mechanically decidable. A **gate** is a small program
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
  A **targeted run** is `run-gates.sh --only <name>...` (§run-gates), and it is a
  spelling rather than a mechanism because these two knobs already compose into
  one: point the positional at a scratch directory holding a one-line
  `gates.list`, and set `GATE_SDK_VERBOSE` to restore the per-gate banner the
  runner suppresses on a pass. One gate's verdict and its clean-path output are
  then reachable with the `GATE_SDK_KNOB_*` bridge `gate_command` emits still
  intact, rather than hand-exported at the call site. That composition stays the
  answer for a caller assembling a registry that is **not** a subset of the
  default one — a gate the consumer has not registered, or one shadowed for the
  run; `--only` is the ergonomic form for everything inside it.
- the consumer's own `check-*.sh` gates (copy-edits of
  `templates/check-skeleton.sh`).
- `gate-tests/` — the consumer's fixture tree (see §run-gate-tests).
- `git-hooks/` — the generated `pre-commit` (see §gen-pre-commit) and any
  hand-written hooks.
- `gate-sdk-config.sh` — optional persistent config: a sourced shell file that
  sets any `GATE_SDK_*` layout knob so the override outlives the shell that set
  it (see the loader paragraph below).
- `graph-vocab.sh` — optional rule content for `check-graph` (see there).
- `graph-theme/` — optional consumer theme **directory** for `check-graph`'s
  emitted artifact, its parts read as data (see there).
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

Environment overrides, all optional: `GATE_SDK_GATES_DIR` (default `scripts`; the
crate names no gates directory anywhere, the owner sentinel `-` standing in its
place on `--list` — so a reader looking for where the binary learns this repo's
layout finds the answer *nowhere* stated rather than merely absent, and the one
place the crate resolves the directory at all is the owner unit test, through
this knob's own resolver — §check-gate-substrate-parity),
`GATE_SDK_TESTS_DIR` (default `<gates-dir>/gate-tests`), `GATE_SDK_HOOKS_DIR`
(default `<gates-dir>/git-hooks`), `GATE_SDK_WORKFLOW_DIR` (default
`.workflow`; the directory's two-tier membership rule, header form, and
extension rule are §The workflow directory — resolved in `lib/gate.sh` rather
than inline at its readers, for the same reason the two `check-kit-registration`
document knobs below are: the governed-comment corpus takes this directory's
tracked tier, and a knob the owning kit's library does not define is the config
bridge's undeclared-knob refusal, so a compiled member declaring it would fail-close on
every invocation), `GATE_SDK_GRAPH_ARTIFACT` (default
`<gates-dir>/CHECK-GRAPH.html`; the emitted coupling-graph artifact's path,
read by `check-graph` assertion E — set it to republish the artifact elsewhere,
e.g. a served docs page), `GATE_SDK_TMP_DIR` (default `.tmp`; resolved in
`lib/gate.sh` rather than only inline at its readers, for the reason the workflow
directory above carries — the battery runner is a compiled arm and declares it,
and a knob no kit library defines is the config bridge's does-not-define
refusal), `GATE_SDK_JOBS`
(default unset = `available_parallelism()`; the battery's worker count, `1`
restoring a serial run — env rather than argv because worker count is execution
configuration and changes no member's membership, see §run-gates),
`GATE_SDK_VERBOSE`
(default unset = quiet green; any non-empty value restores the full per-gate
banner roll on `run-gates.sh` and the generated hooks — see §run-gates),
`GATE_SDK_QUEUE_FILE` (default `TASK-QUEUE.md`; resolved in `lib/gate.sh` rather
than inline at its readers, for the third occurrence of the reason the workflow
directory above and the two `check-kit-registration` document knobs below carry —
a compiled member valving the queue out of its corpus declares this knob, and a
knob no kit library defines is the config bridge's undeclared-knob refusal **whatever
prefix its name carries**, so an environment-only override would fail-close on
every invocation), `GATE_SDK_AGENT_FILE` (default
`CLAUDE.md`; the always-loaded agent file a consumer's harness reads — set it to
`AGENTS.md` under an agent-file harness, and `check-root-tiering`'s built-in
allowlist accepts that file at root), `GATE_SDK_GRAPH_THEME_DIR` (default
`<gates-dir>/graph-theme/`; the optional consumer theme directory whose part
files `check-graph`'s emitter inlines verbatim to bring host-site tokens/chrome
into the emitted artifact — see there; it **replaces** the retired
`GATE_SDK_GRAPH_THEME` sourced-function seam, which the gate now refuses on
rather than ignoring), `GATE_SDK_GRAPH_MAX_EDGES` (default `100000`; the render
cap the emitted page declares and `check-graph` asserts against — see there),
`GATE_SDK_GRAPH_EXTERNAL_REFS` (default empty; space-separated URL
prefixes the `check-graph` external-ref assertion sanctions beyond the
kit-seeded mermaid import — a consumer whose theme chrome links absolute URLs
lists their prefixes here — see §check-graph), `GATE_SDK_CORE_FILES_FILE` (default
`<gates-dir>/core-files.list`), `GATE_SDK_IDENTITY_FILE` (default
`<gates-dir>/identity.conf`), `GATE_SDK_GIT_EMAIL_FILE` and
`GATE_SDK_GIT_REMOTES_FILE` (both default empty; each names a file standing in
for one thing the clone itself says — `git config user.email`, and the
`<remote> <url>` set — so an empty value falls through to the live `git` read
that is the production path — see §check-identity), `GATE_SDK_GH_HOSTS_FILE`
(default empty; the GitHub CLI's persisted hosts file, the account kind's
actual, where empty means *derive it* — the CLI's own config-dir variable
first, the XDG config home second — and never *no file*, the derivation living
in the member because it reads `$HOME` — see §check-identity),
`GATE_SDK_GH_HOST` (default `github.com`; the host whose block that file is read
for, mirroring the CLI's own host variable rather than taking a third manifest
field — see §check-identity), `GATE_SDK_PRUNE_DIRS` (default
`target .git node_modules .tmp gate-tests worktrees`; it **replaces** the default
set rather than extending it — see §lib/gate.sh for the members' rationale),
`GATE_SDK_PRUNE_EXTRA_DIRS` (default empty; space-separated directory basenames
appended to the resolved prune set, whether that set came from the default or
from `GATE_SDK_PRUNE_DIRS` — the additive counterpart that lets a consumer add a
member without maintaining a copy of the kit default that drifts; *removing* a
default member is still the replacing knob's job, because an append cannot
subtract — see §lib/gate.sh), `GATE_SDK_GRAPH_VOCAB` (default
`<gates-dir>/graph-vocab.sh`), `GATE_SDK_KIT_DIRS` (default: gate-sdk + its
siblings holding a `checks/` or a `smoke/`), `GATE_SDK_ROOT` (default: the
vendored `gate-sdk/` resolved beside the sourcing script — the root a
consumer-copied gate sources `lib/gate.sh` from and the anchor kit roots
relativize against), `GATE_SDK_ROOT_ALLOWLIST` (default
`<gates-dir>/root-allowlist.list`), `GATE_SDK_REGISTRY_DOC` (default `README.md`)
and `GATE_SDK_RUNNER_DOC` (default `README.md`) for `check-kit-registration`
— both resolved in `lib/gate.sh` rather than inline in the check, so the config
bridge can carry them to the compiled member (see there),
`GATE_SDK_MSG_PATTERN_FILES` (default
`<gates-dir>/msg-patterns.list`; space-separated, each tracked and required —
fail-closed when missing), `GATE_SDK_MSG_PATTERN_FILES_LOCAL` (default
`<gates-dir>/msg-patterns.local.list`; gitignored, skipped when absent so a
fresh clone without the operator's private list still commits),
`GATE_SDK_COMMIT_TYPES` (default
`feat fix refactor perf docs test build ci chore style`; the shared
commit-type roster — see §check-commit-subject — resolved onto the knob's own
name in `lib/gate.sh` rather than inside `gate_commit_types`' expansion, the
same reason the two document knobs above are: a default the bridge's `declare
-p` cannot find is its undeclared-knob refusal, so the compiled member would
fail-close on every invocation), `GATE_SDK_EXEC_GLOBS`
(default `*/checks/*.sh */kpis/*.sh */bin/*.sh` plus the computed
`<gates-dir>/check-*.sh` and `<gates-dir>/kpi-*.sh`; the path globs whose
tracked `*.sh` members `check-exec-bit` holds to index mode `100755` — see
there), `GATE_SDK_EXEC_PRUNE` (default `gate-tests fixtures templates smoke`;
the path segments whose subtrees `check-exec-bit` exempts — see there), `GATE_SDK_ENFORCE_SCAN_DIR` (default `.`; the enforcement map's
monitor-marker scan root — see §enforcement-map), and
`GATE_SDK_LINT_EXTRA_DIRS` (default empty; space-separated directories whose
direct `*.sh` members join `check-shellcheck`'s derived scan set — the seam for
a shipped script that sits under no kit root — see §check-shellcheck),
`GATE_SDK_PROGRAM_FLOOR` (array, default the POSIX/coreutils set the battery
already rests on plus `bash`, `sh` and `git`; the programs the payload is
entitled to assume present, so a command-position word among them is not a
criterion-7 requirement — the default is written once in `lib/gate.sh`, see
there and §port-blockers), and
`GATE_SDK_NATIVE_BIN` (default **computed**: `native/target/release/checkwright-gates`
with the **host's** executable suffix appended, `gate_exe_suffix`'s no-argument
form — so `…/checkwright-gates` everywhere but a Windows host and
`…/checkwright-gates.exe` there. This default is the one place in the tree where
a suffix-less cargo artifact would otherwise be assumed, which is why the suffix
is derived here rather than at any reader. A consumer pinning the knob explicitly
keeps its exact value; only the default moves. Stated as prose because a
computed default has no literal for `check-knob-default-coupling` to couple to;
the multi-call binary `gate_command` dispatches a `.gate`-declared member to —
see §lib/gate.sh; also the binary §check-gate-binary-fresh asks for its baked
source stamp). Its default is a **stable relative path** deliberately: the
generated pre-commit hook persists the emitted argv, and a machine-specific
absolute path baked into a tracked hook would make `check-graph`'s byte-freshness
comparison machine-dependent. **A vendored consumer's value is set for them**, to
the binary's place in their gates directory, by the installer writing this knob
into their `gate-sdk-config.sh` (installer/README.md §The gate binary) — the
config seam exists so a value can be relocated without the default moving.
Deriving the default from `GATE_SDK_GATES_DIR` instead was weighed and refused:
it would silently relocate the binary for every existing reader, and make this
repo's own layout the exception to a convention whose stated rule is that this
repo's layout *is* the default. And `GATE_SDK_NATIVE_SRC` (default **derived**
from `GATE_SDK_NATIVE_CRATE` as `<crate>/src`, so this repo's `native/src`;
the implementation tree §check-gate-substrate-parity assertion D holds free of
manifest-class annotation — a **path, not a language**, so the knob assumes
nothing about what implements a ported gate). And `GATE_SDK_NATIVE_CRATE`
(default `native`; the crate root that owns the build manifest, held outside
every kit root by §check-gate-substrate-parity assertion E, and the tree whose
tracked source §check-gate-binary-fresh hashes into the stamp it compares. Read
through `gate_native_crate`, its one shell home — §lib/gate.sh). `GATE_SDK_KNOB_<NAME>` joins this roster as the one entry that is **not a
consumer knob**: it is a *dispatch-time convention*, written by `gate_command`
into the argv it emits and read by the binary's knob reader (§lib/gate.sh, the
array-knob config bridge, whose three value shapes — a tab-joined element list, a
scalar as its one-element case, and an associative knob's sorted `<key>=<value>`
pairs — that one name spans). **No consumer sets it, and saying so is what stops it
being documented as one** — it has no default, it is not read from the config
seam, and a value found in a consumer's environment is overwritten by the
dispatch that emits it. It adds no configuration surface: the bridge carries
whatever the consumer's own library resolved for a knob the consumer already
owns. The two native
path knobs stay distinct because they answer different questions:
`GATE_SDK_NATIVE_SRC` names the implementation tree a gate's rule lives in,
`GATE_SDK_NATIVE_CRATE` the root that would carry that tree with it if it moved,
and only the second decides whether the implementation sits inside the vendoring
set (§Consumer payload). And `GATE_SDK_NATIVE_TARGETS_FILE` (default **derived**
from `GATE_SDK_NATIVE_CRATE` as `<crate>/targets.list`, exactly as
`GATE_SDK_NATIVE_SRC`'s default is, so the crate's location keeps one owner; the
target roster §Consumer payload rules the platform-support surface, read through
`gate_native_targets` — see §lib/gate.sh). And
`GATE_SDK_NATIVE_PUBLISH_WORKFLOW` (default `.github/workflows/publish.yml`; the
workflow §check-gate-substrate-parity assertion F holds roster-derived and
one-producer-per-digest — a consumer whose release rides elsewhere points the
knob at it, and a consumer with no such workflow is reported rather than red,
because a publish path that does not exist is not one to audit). The roster and
the workflow are **consumer config, never kit literals**: the knob, the line
grammar and the assertions are gate-sdk mechanism, while *which* platforms a
project commits to is that project's own support commitment — a kit literal
spelling one project's would ship it as everyone's (CLAUDE.md §The provenance
seam). And `GATE_SDK_CARGO_TARGET_DIR` (default **derived** from
`GATE_SDK_NATIVE_CRATE` as `<crate>/target`, cargo's own placement, so the
warm cache `bin/build-native.sh` fills is the one §check-crate-arms reuses;
that derivation is performed **in `lib/gate.sh`** beside the three crate-adjacent
knobs above, which is where its reader's port had to move it — a default written
inline at the use site is invisible to the config bridge's `declare -p` and is
its undeclared-knob refusal, and this sentence had described the derivation for
longer than the code carried it).
Its only non-default reader is that gate's fixture pair, which redirects it out
of the tree so a fixture run leaves no build products beside a fixture manifest;
defaulting it to scratch instead is refused, because it would cold-build the real
crate on every battery run to buy a hermeticity the real tree does not need.
Paths are
repo-root-relative; every entry point `cd`s to `git rev-parse --show-toplevel`
before resolving them. That sentence states **shape and mechanism and never
dialect** — which spelling a root arrives in, and who owes the conversion, is
§The path-dialect contract below.

`lib/gate.sh` auto-sources the consumer config seam on load, so every gate sees
the same knob resolution, whether it sources the library itself or is dispatched
through it as a `.gate` member: `GATE_SDK_CONFIG_FILE` when
set — and a set-but-missing path exits 2 rather than silently running on
defaults (an operator typo must not pass as clean) — else
`<gates-dir>/gate-sdk-config.sh`, sourced only when the file exists — a
zero-config consumer is unaffected. Env vars still win (the config file sets a
default the invoking shell may override), but the file is how an override
*persists*: an env-only knob dies with the shell that exported it, so a
consumer that must relocate a layout knob for every session sets it here. The
one knob the file cannot set is `GATE_SDK_GATES_DIR`, which locates the file
itself — it stays env-or-default (a config file cannot name its own directory).

## The path-dialect contract

A **root** is a path this tree passes between components, and the sentence above
says which *shape* one has and never which **spelling**. On a host carrying two
path dialects — an MSYS or Cygwin shell over a Windows filesystem — that omission
is not academic. A root can arrive drive-lettered and backslash-separated, every
downstream resolution then composes a string no filesystem answers to, and the
battery reports its **entire** roster unresolved rather than failing at the
crossing. This section states the three things that make a call site judgeable at
all: which dialect a root is in, where the boundary is crossed, and who crosses
it.

**The declared dialect is per-substrate.** A root is spelled in **its own
substrate's** dialect — forward separators, absolute by that substrate's rules —
and is converted into it **once, at the producer**. MSYS bash holds `/c/repo`; a
`*-windows-msvc` binary holds `C:/repo`. What carries the weight is a property of
a value *anywhere inside the tree*, not only of the point that produced it: no
value inside the tree is in a foreign dialect, and no value is normalized twice.

Two dialects rather than one because **each substrate's correct dialect is the
other's defect**. The crate's crosser is where that becomes concrete:
`normalize_abs` keeps `path_root(abs)`, so `C:\repo` becomes `C:/repo` —
separators normalized, drive letter preserved (§The crate's crosser: "the composed
result carries the **input's own** root"). Teaching it to strip the drive is
**refused**, and the refusal is the substance of this clause: a `*-windows-msvc`
binary reaches the filesystem through `std::path`, which cannot resolve `/c/repo`,
so there the drive letter is not residue but the only absolute spelling that
works. An MSYS bash process wants the opposite — its own `getcwd(3)` answers
`/c/repo`, and every sibling path such a shell derives carries that spelling, so a
root spelled `C:/repo` compares unequal to a path spelled `/c/repo` while naming
one directory. No single spelling is available to declare.

**The split leaks nothing, because roots cross the shell/crate boundary
relative, never absolute.** `walk::kit_roots_abs` re-absolutises each bridged root
against the crate's own cwd — which is §lib/gate.sh's `GATE_KIT_ROOTS_HERE` rule,
each root spelled relative to the invoking directory, read here as a dialect
guarantee rather than only as the public-file constraint it was written for. No
value arrives in a substrate whose dialect it was not spelled
in, so the per-substrate declaration costs nothing at the seam.

**The boundary, and who crosses it.** The dialect boundary is crossed exactly
where a value enters from a **platform-native producer**: `git rev-parse
--show-toplevel` under Git-for-Windows (a native Windows binary, so it answers in
Windows spelling even to a POSIX shell), `std::env::current_dir()` in a
`*-windows-msvc` binary, and an npm bin shim's basedir. The **crosser** is the
entry point that reads such a producer, and normalizing there is the crosser's
obligation. A shell builtin is *not* a platform-native producer — but which
builtin, and in which form, is the whole of the shell crossing, so it is spelled
out rather than gestured at.

**The shell crossing idiom, and why `pwd -P` is the half that crosses.** bash's
`cd` with an **absolute** argument sets `PWD` from the argument itself and never
calls `getcwd`. So `cd 'C:/repo'; pwd` prints `C:/repo` straight back,
unconverted: a migration written `cd … && pwd` changes nothing while looking
exactly like the fix. `pwd -P` calls `getcwd(3)`, which under the MSYS runtime
answers in the MSYS spelling. **The crossing lives in `-P`.** In the
cwd-preserving form most sites take:

    ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )"

and, where the entry point means to *be* at the root, the two-line form that keeps
the site's existing refusal arm verbatim:

    cd "$(git rev-parse --show-toplevel 2>/dev/null)" || { …refuse exactly as today…; }
    ROOT="$(pwd -P)"

Two properties travel with the idiom rather than being left to a reader. `pwd -P`
also resolves symlinks — a behavior change accepted rather than overlooked, and a
no-op in practice because `git rev-parse --show-toplevel` already answers a
physical path. And the `|| pwd` hedge several sites carry survives inside the
substitution untouched: it is a missing-repository guard, the clause below rules
that it confers nothing against dialect, and the idiom neither needs it gone nor
is weakened by it.

**No shared shell normalizer is introduced, and the refusal is load-bearing.** No
kit's `lib/` holds one, so a helper would be new rather than a second copy — but
the corpus's three families source three different libraries and two resolve their
root before sourcing anything at all, so a shared helper buys a cross-kit
dependency to save a one-line idiom, against the provenance seam and for no error
class `check-path-dialect` does not already catch. The idiom needs no name:
`scripts/producer-liveness-reader.sh` and `scripts/pack-installer.sh` both write
it already, neither as a dialect measure, which is the evidence that it is the
shape a shell author reaches for unprompted.

A value already inside the tree is in the declared dialect by the clause above and
is **never re-normalized**: a second
normalization is how a contract decays into a ritual, and it erases the one place
a reader can look to find where the conversion happened.

**The judging predicate is about consumption rather than about the producer.**
This is the clause that makes the contract usable, and the one a reader gets
wrong:

> A root consumed only by `cd` is **dialect-tolerant**. A root consumed by
> **string concatenation** is **dialect-exposed**.

`cd` accepts either spelling on an MSYS host; `"$ROOT/sub"` does not. So the audit
question at a call site is never *where did this root come from* but *what is done
with it* — provenance is a chase across components, consumption is local and
decidable by reading one line. An exposed site is not thereby broken: exposure
says its value must have reached it through a crosser, and the two halves are
judged separately.

**A `|| pwd` fallback confers nothing, and believing otherwise is the trap.** It
fires only when `git` **fails**; on MSYS `git` **succeeds**, in the wrong dialect.
It is a missing-repository guard and must never be read as a dialect one — which
is why a bucket of call sites carrying it does not fall out of an audit by
construction.

### Porting to Rust does not retire dialect exposure

A clause of the contract rather than a footnote to it, because the inference it
kills is the intuitive one: *shell is the dialect-fragile substrate, so a ported
file's dialect problem goes away.* That is **false in this crate**, and the defect
this contract was written from is the proof — the port did not inherit the
exposure, it **created** it. `native/src/walk.rs` composed roots with `String`,
`split('/')` and `format!` rather than with `Path`/`PathBuf`, so it
re-implemented by hand the POSIX assumptions `std::path` would have carried. Rust
is dialect-safe only where `Path` is actually used, and a port reaching for string
composition — as a port of a shell file naturally does, because the shell it is
porting composed strings — is exactly where a *new* exposure is born. The reader
this clause is for is the session porting the next kit (§Porting a gate to the
binary substrate), at the moment it assumes the port retires the question.

### The crate's crosser

`native/src/walk.rs` holds the crate's single owner of absoluteness, `path_root`,
and it is also the crate's **sole platform-native producer**: `std::env::current_dir()`,
the `git rev-parse` spawn and `std::fs::canonicalize()` live there and nowhere
else in the crate. Ownership of absoluteness without ownership of the producers
only moves the question one call back, which is what this monopoly closes.

- **`walk::cwd()`** is public and **normalizing** — `normalize_abs` of
  `std::env::current_dir()`. It was private with one caller, `kit_roots_abs`,
  which normalized downstream through `abs_against`; promoting it moves the
  obligation to the source rather than closing a live hole.
- **`walk::toplevel()`** is the crate's only `git rev-parse --show-toplevel`
  spawn, its answer passed through `normalize_abs` on the way out.
  Git-for-Windows is a native Windows binary and answers in Windows spelling even
  to a Rust process, so this producer crosses for the same reason `cwd()` does.
  It has **two refusals, kept apart because callers report them differently**: a
  dead `git` is `proc::run`'s own spawn message, propagated; a directory outside
  a work tree is the absent answer. `toplevel_opt()` returns that pair as
  `Result<Option<String>, String>` for the five callers that distinguish them,
  and `toplevel()` is the convenience form that folds the second into
  `not a git repository`. Both are relocated rather than invented, so no caller
  gains a failure mode it does not already handle — `fresh::toplevel()` keeps its
  name, its two callers and its own sentence by suffixing
  `— the emitter anchor cannot be resolved`.
- **`walk::toplevel_in(dir)`** asks the same producer from another directory.
  It exists because two callers compare a `-C` answer against a bare one, and an
  uncrossed side would make that a comparison between dialects rather than
  between directories.
- **`walk::canonicalize()`** is the crate's only `std::fs::canonicalize`, and the
  one producer here that **does not convert** — see the UNC clause below.
- **There is one normalizer.** `check-stage-evidence`'s `norm()` was a second
  implementation that split on `'/'` only, so it could not repair a
  backslash-spelled root — a normalizer that silently does nothing is worse than
  none, and the single-owner rule is what forbids it. It is retired onto
  `walk::normalize_abs` and survives only as the frame shift that removes the
  leading separator, because its result is also spliced into a `HEAD:`-prefixed
  revision.

`abs_against` and `normalize_abs` are the pure half — functions of their inputs
alone, converting into the declared dialect once, on entry:

- absoluteness is a **two-dialect** question — a separator-rooted path and a
  drive-rooted one are both absolute, and a leading-slash test answers *false* on
  the second, sending an already-absolute root down the join-onto-cwd arm;
- segments split on **either** separator, so a backslash run is not one segment;
- the composed result carries the **input's own** root, never an unconditional
  POSIX slash prepended onto whatever arrived.

A rootless input keeps the pre-repair reading, treated as separator-rooted,
because the contract puts no relative path here and the repair is scoped to
dialect rather than to that caller error.

**UNC and the extended-length prefix are out of scope, and one surface does
produce one.** `path_root` reads `\\?\C:\repo` as separator-rooted, so
`normalize_abs` would return `/?/C:/repo` — a mangling, not a conversion. On
Windows `std::fs::canonicalize` answers in exactly that spelling, which is why
`walk::canonicalize()` is the one producer that hands its answer back unconverted
rather than crossing. That is safe on the evidence and only on it: both callers
compare its output **against its own output only** — `check-gate-exemption-tasks`
matches two canonicalized directories, `check-surface-duplication` takes a
basename — so the spelling is symmetric and unobservable. No third caller may
assume that. Which spelling Windows actually returns, and whether a strip rule is
owed, is not decidable from a Linux host and is filed rather than guessed.

`native/src/registry.rs` is the consumer whose failure was the observed one: it
appends the `checks` segment to each resolved root and enumerates descriptors from
it, so a root resolving to nothing takes **every** gate with it. That is this
pair's honest blast radius — it has no small one, which is the argument for
keeping the repair a change to how two functions map inputs to outputs and to
nothing about when they are called.

### How the claim is held, with no oracle that can run it

There is **no CI oracle** for this, and the contract says so rather than implying
one: the Windows leg runs the installer smoke, `continue-on-error`, and dies
before the battery runs, while the Ubuntu leg only cross-compiles. So the arm is
local and host-independent, on the mechanism §Fail-closed contract already
established for `on_path` — the platform-dependent input is **injected**, which
makes the decision a pure function a Linux host can exercise, and the assertion is
**paired with a control** that fails if it would otherwise pass vacuously. Here
the injected input is `cwd()`'s spelling, and the control is the pre-repair
composition itself, kept in the test and asserted to still reproduce the string
the Windows leg printed. `on_path`'s third part — a source scan standing in where
no behavioural call could run — is **not** owed here: once the spelling is
injected the composition rule is fully reachable, and a scan would assert shape
where behavior is available.

**The honest limit, so a green board is never read as a Windows run.** What these
tests exercise is the composition rule, on the string the windows-msvc leg
reported. That `std::env::current_dir()` answers in Windows spelling there is the
**injected premise**, held by the observation this contract was written from and
by nothing this repo can run.

### Worked dispositions, because a judged-safe site and an unjudged one look identical

Recording a verdict is the deliverable, not only changing what a verdict
condemns — an unjudged site and a judged-safe site are indistinguishable on the
page, and the contract's value is precisely that difference. **Where that verdict
is recorded is at the site**, as the `spec:` citation `check-path-dialect` reads,
never as a roster here: a per-site enumeration in this section goes stale on every
ported file and duplicates a record the gate already reads.

What this section keeps is the one verdict a reader most needs a worked instance
of, *exposed-but-satisfied* — a site whose consumption **is** dialect-exposed and
which is nonetheless owed no change, because its value was already crossed:

- `scripts/producer-liveness-reader.sh` resolves `git rev-parse --git-common-dir`
  inline with `pwd -P`, then **concatenates** the result (`${_plr_common%/*}`, then
  `"$_plr_main/$GATE_SDK_NATIVE_BIN"`). Exposed consumption, satisfied at its own
  crossing: **no change owed**, and adding a normalization downstream would be
  exactly the re-normalization the boundary clause forbids.
- drift-kit's `drift-report.sh` carried the same verdict on a second root, `KIT`,
  crossed by `cd "$(dirname …)" && pwd`. **That site is gone**, deleted with the
  shell collator by the 2026-08-29 drift-kit cut, and it is kept in the past tense
  rather than struck out because what it teaches is the predicate rather than the
  file. Read with the clause above, it also dates itself: `pwd` without `-P` is
  today a no-op, so a site written that way is uncrossed, not satisfied.
- The arm that replaced it composes `<kit-root>/kpis/<name>.sh` from the resolved
  kit-root set — a **new** dialect-exposed site, born of the port exactly as
  §Porting to Rust does not retire dialect exposure says it would. It is
  **satisfied**: those roots reach it through `walk::abs_against`, the crate's own
  crosser, so they are already in the declared dialect. `registry.rs`' `checks`
  append is the same shape and the same verdict.

`check-path-dialect` is what holds the claim across the remaining port, and what
it asserts is bounded: every producer in the corpus crosses, or carries a recorded
verdict. It does **not** assert that the crossing works — that rests on the
injected premise §How the claim is held names — and it holds no coverage floor, so
a green verdict is the absence of an uncrossed producer and nothing more.

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

## The bin/-tool contract

A kit's `bin/` tools are shipped executables that are explicitly **not gates**:
they take caller-supplied arguments, and several of them *capture* — appending
what they were handed to a durable, committed surface. The gate model below
governs none of them, so the one authoring rule every kit's tools share is
stated here, beside §check-exec-bit's tree-wide `*/bin/*.sh` invariant.

**A `bin/` tool whose positional arguments are free text validates their shape,
not only their arity.** Free text is an uninterpreted caller-supplied string; an
argument drawn from a known set is not free text, because validating membership
already validates shape. Three behaviors:

- `-h` / `--help` as the first argument prints the tool's usage on **stdout**
  and exits **0**. Usage on a successful help request is output, not a
  diagnostic.
- A positional argument beginning with `-` that is not a recognized option is a
  **refusal** — usage on stderr, exit 2.
- `--` ends option processing, so every remaining argument is taken as free text
  however it is spelled.

An arity check is not a shape check. A tool taking exactly one free-text
argument accepts `--help` *as* that argument, and a tool taking two accepts a
flag in either slot; arity makes the single-argument case worst and the others
merely quieter, never safe. A tool that captures turns the defect into a written
record that reads like a real filing — attested three times across three
sessions, each writing a flag into a committed, boundary-blocking inbox at exit
0 — so the rule binds hardest there. But it binds on every free-text tool,
because the help half is discoverability and its cost is measured too: a session
hunting for a mode ran a stage writer with `--help`, got `'--help' is not a
lifecycle stage` in place of usage, and went three guards deep working around a
contract the usage text would have told it did not exist.

The `--` escape is not decoration: without it the refusal makes a legitimate
filing unfileable, and this rule's own subject matter is the instance — recording
that a `--list` argument was captured takes
`bash lifecycle-kit/bin/file-gap.sh -- "--list is captured at exit 0"`.

**No gate reads this rule, and that is ruled rather than deferred.**
§check-exec-bit's corpus is the whole `*/bin/*.sh` set, and a gate over it could
assert only the weak static shape — does the file contain a `--help` branch —
which passes a tool that prints usage and captures `--list` anyway. The
predicate that matters is behavioral, and the precedent for behavioral coverage
of a `bin/` tool is already ruled and shipped: lifecycle-kit/SPEC.md
§bin/enter-stage.sh rules `--simulate` advisory tooling owed no fixture pair and
exercises it end-to-end in `smoke/`. Each member's coverage follows it, and
enforcement-first's own clause — removing the duplication outranks gating it —
points at one stated rule plus behavioral coverage rather than a static scanner
that would green the case it was bought for.

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

**On the binary substrate the defect is closed by construction, not by review.**
A ported member spawns nothing itself: the crate's one spawn site is
`native/src/proc.rs`, whose `run` returns `Err` for a **spawn** failure — in the
same "could not run" words above — and a `Completed` otherwise, and `Completed`
hands out stdout only through an accessor that has already read the exit status.
So "capture `stdout`, ignore `status`" has no spelling, and a program that could
not be run is an exit-2 refusal rather than an empty capture read as clean.
The wrapper is proved the way the shell helper is, directly rather than through a
member's fixture pair: unit tests over a spawn that never happened, a child that
exited non-zero, and one that succeeded. A further test holds the routing — the
`Command` spelling is asserted absent from every module under
`native/src/gates/`, the roster shape §check-reads-couples' unit test B uses for
filesystem walks — so each new spawning member inherits the property instead of
re-buying it. That corpus is the gate modules and stops there: the `#[cfg(test)]`
helpers bridging to the shell library from `walk.rs` and `main.rs` check their
own status already, and pulling them in would make the production wrapper carry
a cwd and an env nothing in production reads. `native/src/actions.rs` is shared
gate mechanism sitting **outside** that corpus by construction and is named here
because the move that put it there is the easiest reader to miss: it spawns
nothing, and the property is preserved not by the roster test but by the
single-spawn-site rule above — `proc.rs` is where a spawn would have to be added,
and adding one there is what the corpus test is watching. Widening `proc.rs` is how a member
that needs more of the child's result gets it; building its own `Command` is what
the test refuses.

**The same contract binds the *dispatcher*, and there the refusal is raised
inside `gate_command` — which is a trap for its callers.** A `.gate` member whose
binary is absent or not executable is a harness error, so `gate_command`
(§lib/gate.sh) writes its own naming stderr and **`exit`s 2 from inside itself**
rather than returning. A caller that invokes it through a **process
substitution** — `mapfile -t argv < <(gate_command …)` — never observes that
status: the `exit` kills only the subshell, `mapfile` binds an empty array, and
the caller reads an empty argv whose *only modelled cause* is `gate_command`'s
`return 1` for a member that resolves in no check dir. The true diagnosis is then
overwritten by a false one, and a session is told the gate does not exist when it
exists and could not be built. **So a caller captures `gate_command`'s status —
a command substitution, never a process substitution** — and branches on it:
status 1 is the resolves-in-no-check-dir refusal it may name itself, and any
other non-zero status is a failure `gate_command` has already named on stderr and
is propagated without a second, contradicting sentence. Both in-tree dispatchers
take that shape (`gate-sdk/bin/run-gates.sh`, and this repo's front end
`scripts/gate-exec.sh`, whose obligation evidence-kit/SPEC.md
§check-evidence-manifest owns).

**The wrapper contract: a member whose rule *is* an external program refuses with
its own message, at the shell form's own point in the order.** Criterion 7's
program-is-the-rule class ports as a wrapper (§The port-candidate criteria), and
`run`'s `Err` arm alone does not discharge it. Two things it does not carry, both
in `proc.rs` so a cohort of wrappers buys them once:

- **`on_path(program)`** — bash's `command -v <prog>` reduced to *is there an
  executable of this name on PATH*. It exists so the refusal is the member's own
  documented text rather than a generic spawn-failure string, and so it fires
  **where the shell form fired it**. That ordering is load-bearing rather than
  cosmetic: `check-shellcheck` probes the linter before it globs its targets, so a
  tree with nothing to lint and no linter reports the *linter*, and catching the
  spawn failure instead would report the empty corpus. The probe's place in the
  order is per-member and not a class rule: `check-action-run-shell` validates its
  positional scan root *first*, so an absent root reports the root even with no
  linter installed. What the class fixes is that the point is the shell form's,
  whatever it was. A refusal message is a
  documented surface — a session debugging its PATH reads the message, not the
  exit code. `run`'s `Err` arm stays the backstop for a program that vanishes
  between the probe and the spawn. **Two platform axes decide whether it answers
  at all, and getting either wrong makes it report a program the host has as
  absent** — a wrapper refusing on an installed program, the exact false verdict
  the class exists to prevent. The **separator** is `std::env::split_paths`'s,
  never a literal `':'`: a Windows `PATH` separates on `';'` and its entries carry
  drive-letter colons, so a literal split shears every entry past the first into a
  fragment. The **name** is not the bare program: a Windows toolchain installs
  `<program>.exe` and npm's own bin shape installs a `.cmd` shim, so the probe
  tries the program with each `PATHEXT` extension appended, falling back to
  `.COM;.EXE;.BAT;.CMD` when the variable is unset or empty — read from the
  environment rather than appended as a literal, on `gate_exe_suffix`'s ground
  (§lib/gate.sh), and owned by `exe_candidates` alone so no call site spells an
  extension. That is a **different question** from `gate_exe_suffix`'s — what an
  *installed* program may be named, not what a *built artifact* is named — which
  is why the two substrates hold separate single owners rather than one. The bare
  name stays a candidate, so a caller naming `cargo.exe` and a Unix host both
  resolve through the same loop. The resolution is therefore a **pure function of
  (PATH, PATHEXT, an existence predicate)**: its Windows arms cannot execute on the
  host that develops them, so a fake predicate is the only oracle a unit test here
  has. The separator has none even so — `split_paths` compiles to the host's rule,
  so what a test can pin is that the literal has not come back, and the arms
  themselves are exercised by the Windows CI leg or not at all. **This is the
  cannot-exercise-locally doctrine's first instance and it now has a second** —
  §The path-dialect contract's root resolution, held by the same injected-input
  shape and the same paired vacuity control, differing only in that its decision
  *is* behaviourally reachable once the input is injected and so owes no source
  scan. The mechanism is stated here and cited there rather than restated.
- **`run_merged` and `Merged`** — the `2>&1` capture a wrapper's shell form takes.
  `Completed` withholds stdout unless the status succeeded, which is right for a
  reader and wrong for a wrapper: for a linter the **non-zero** run is the one
  whose report must be printed. `Merged` therefore reads its report whatever the
  status, and the false green is closed on the other side instead — `succeeded()`
  reads the status, so a clean line is unreachable from an empty capture and
  emptiness is never the branch. The merge is two handles on **one** file
  description (`try_clone` is `dup`), the technique `dispatch` already uses, so
  the two streams interleave exactly as bash's `2>&1` did; reading them as two
  pipes and concatenating would reorder a linter's findings against its errors.
  `Merged` also exposes the child's **exit code**, on `Completed::code()`'s own
  ground and added when the second wrapper needed it: a program that grades itself
  by more than success and failure has to be read that way, and ShellCheck is the
  worked case — 1 is *findings I lint into a report*, ≥2 is *a fragment I could
  not lint at all*, and folding the two into `succeeded()` would print an error as
  though it were a finding list. Its sibling `reported_code()` answers the same
  question for a wrapper that **prints** the code instead of branching on it:
  bash's `$?` spells a signalled child `128 + n`, so a member whose report reads
  `failed (exit N)` keeps the number the shell form printed rather than collapsing
  a killed child to a sentinel. Two accessors rather than one because the grading
  caller needs the *"not gradeable"* `None` that the printing caller cannot use.
- **`run_streamed` and `Streamed`** — a wrapper whose program is a **filter**: a
  body written to its stdin, its stdout read back, its stderr left alone. Three
  things distinguish it from the two faces above and each is forced.
  `run_with_stdin` pipes both directions and writes its whole input before reading
  a byte, so a child that fills the stdout pipe stops draining stdin and both
  sides block — which a shell caller never meets, because its process substitution
  *is* a concurrent reader. Two capture files are deadlock-free without one, and
  the input size that makes this real is measured rather than feared:
  `check-docs-render-fidelity` streams its whole docs corpus, two orders of
  magnitude past a pipe buffer. stderr stays **unmerged**, unlike `Merged`'s: a
  filter's output is a framed stream its caller parses, and folding diagnostics
  into it would corrupt the framing rather than annotate it. And a spawn failure
  is folded into the child's own `code()` as bash's **127**, or **126** where the
  file is present and will not execute — because for this shape the code is a
  number the member *prints inside its own refusal*, so an `Err` arm would produce
  a different message on a different branch and lose exactly the message parity
  `on_path` exists to keep. `stdout()` is ungated for `Merged`'s reason, and the
  false green is closed the same way: the caller reads `code()`, or grades the
  stream by a framing count an empty capture cannot satisfy.

**Who consumes `on_path` is derived, never listed here.** The sections below name
members as instances of a rule, not as a roster, and a reader wanting the live
consumer set greps the crate for the symbol — the arms include non-gate ones, so
a gate roster would not answer it either. Named because the absence of a stated
owner sends a reader to assemble the set by hand, and the assembled set is stale
the next time an arm is added.

**A wrapper's refusal is not always an `on_path` refusal, and the class has both
shapes.** `on_path` serves a member that *tests for* its program;
`check-docs-render-fidelity` **probes its oracle by running it** — two documents
in, two documents back — because for a renderer, present-on-PATH is not the
property the gate needs, and a parser build that loads but cannot parse would pass
a presence test. There the absence surfaces as the probe's own exit status inside
the member's own sentence, which is why `run_streamed` synthesizes the status bash
would have reported rather than routing absence to a second message. Both shapes
obey the one rule the class does have: the refusal fires where the shell form
fired it, and says what the shell form said.

**The fifth wrapper found the third shape: a shell form that fired no refusal at
all, and porting it means diverging deliberately rather than reproducing it.**
`check-producer-liveness` reaches `ps` through `ek_pid_alive`, which discards the
program's 127 into a boolean and reports *not alive* — so with `ps` gone the shell
form prints a clean line, which is precisely the *clean because the program was
missing* vacuity this section exists to close. There is no shell refusal to match,
so the class rule *fire where the shell form fired it* has nothing to say and the
**exit-2 rule decides instead**: the port refuses, on the fallback leg only,
because a `kill -0` that answers never reaches the program. A member reading this
should take the ordering rule as **the constraint when a shell refusal exists**,
not as a licence to inherit a false green where one does not. The divergence is
asserted at the member (evidence-kit/SPEC.md §check-producer-liveness) with its
cost stated, never normalised away, and the parity run reports it as a differing
arm rather than hiding it.

**A wrapper's program can be a shell builtin, and the route to it is `bash` on the
floor rather than a second off-floor dependency.** `ek_pid_alive`'s first leg is
bash's own `kill -0`, which `std` has no spelling for and this crate carries no
`libc` to reach. Three routes exist and only one keeps the requirement set honest:
dropping the leg and probing with `ps` alone makes the program required on every
call rather than on the fallback; spawning `/bin/kill` mints a second off-floor
requirement and moves the refusal to the first leg; `proc::run("bash", &["-c",
…])` reaches the *same builtin the shell form used*, and `bash` is on
`GATE_SDK_PROGRAM_FLOOR`, so the report still counts one program. The declared set
carries both, because unit test A is *observed ⊆ declared* and floor membership is
what the report filters on, not what the registry records.

**Neither half has a fixture representation, so each wrapper's parity run carries
a constructed scenario**: both implementations over the same cases with the
program present, and again with PATH scrubbed of it, comparing bytes and exit
codes. **A wrapper declaring more than one program owes the scenario per program
and for the set** — `check-crate-arms` runs it three ways, `cargo` absent, `rustc`
absent, and both absent, because only the per-program arms show that one program's
absence refuses while the other's does not. A committed case cannot remove a program from PATH — this is the
`# no-fixture:` discharge shape applied to a member that *has* a pair, which
§The port-candidate criteria criterion 2's second worked instance licenses. The
scrub removes exactly the one name and keeps the rest of PATH: emptying it
entirely tests a different absence, since the shell form's own library resolves
kit roots through `realpath`.

**That routing test is also what makes `--needs` trustworthy, which is why the
spawn recorder lives here rather than beside the registry.** `proc.rs` carries a
`#[cfg(test)]` recorder on the shape `walk.rs`'s read recorder has: **every**
spawning entry point — `run`, `run_with_stdin`, `run_merged`, `run_streamed` — notes the program
it is about to spawn, and §The `# graph:` manifest's unit test A reads the note
back after running a member over its fixture cases. A face added to `proc.rs`
that skipped the note would un-verify A silently, which is why the recorder is
part of what widening `proc.rs` costs. It is **test-scoped deliberately** — a production recorder would be state
with no reader — and it is unreachable from a gate module, which is what keeps it
from becoming a runtime surface. Because the routing test already proves no gate
module builds its own `Command`, hooking the two wrapper functions observes every
spawn a member can make; a recorder placed anywhere else would observe only the
spawns that happened to pass through it.

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

**Stated non-target: an invariant whose `bad/` case would itself be unshippable
payload content cannot take the pair form, and takes a producer-side refusal
instead.** The worked instance is *the packed set carries no tracked symlink*
(§Consumer payload). No `good/`+`bad/` gate can hold it, because a `bad/` case
proving the red would have to **be** a tracked symlink inside a kit root —
reintroducing the exact artifact the invariant forbids and shipping it to every
adopter in the payload. `GATE_PRUNE_DIRS` excluding `gate-tests` does not rescue
it: pruning removes the path from a gate's **corpus**, never from the
**payload**, so the fixture would still break the extraction it exists to
prevent. This is written down rather than left as an absence because the rule
above makes the pair mandatory for a shipped gate, so the next reader meeting the
gap will read it as an omission and try to close it. It is not one — the
four-contract shape is what *refuses* the gate here, and `pack_tracked`'s
pre-flight refusal is the form the invariant can take. Same class of disclosure
as §check-tree-terms recording which arms a case dir is structurally unable to
reach.

**The pair is shipping-side, and under §Consumer payload it is the consumer's
whole verification oracle.** It vendors already — `installer/lib/init.sh`
enumerates a kit's payload with an unfiltered `find . -type f`, so the pair
arrives whole. What the ruling changes is its weight rather than its delivery:
with the predicate withheld it is **the only thing a consumer can independently
check**, where alongside readable source it was a convenience. The pair plus
`run-gate-tests.sh` is the entire answer to *does this
binary do what its SPEC section says*. `gate_command`'s substrate-blind dispatch
(§run-gate-tests) is what makes it work across substrates, so nothing new is
built here — only the statement that the property is a contract and may not be
dropped to slim a payload.

**What the pair is evidence *of* differs by tree, and the distinction is not
pedantry.** In this repo it is post-build verification. In a consumer tree it is
**post-install acceptance evidence**: nothing is built there, the artifact
arrives prebuilt and digest-verified, so the question is not *did my build come
out right* — that question does not exist for a consumer — but *does the
artifact I was given behave as the specification I was given says it behaves*,
answered against cases the publisher is held to by the same meta-gates. The
digest answers **which** artifact arrived; the pair answers **what it does**.
The payload ships an answer to both, which is the whole of what replaces reading
the rule. It is also the consumer-side answer to the honest limit §Meta-gate
conservation for the binary substrate records: the crate-side verification of a
gate's declared read-set runs where cargo runs, and that is never a consumer
tree.

**Ruled: the pair ships.** The open question was whether opacity extends to it,
since a `good/`+`bad/` pair shows an agent what passes and what fails. It does
not extend. Fixtures disclose a gate's **shape**, never its **predicate**, and
the pair is the consumer's only parity oracle for a binary they cannot read;
withholding it would spend the trust story to buy marginal opacity, since the
help text and the SPEC section ship regardless.

**A ported member's pair exercises every arm of the corpus derivation it ports,
not merely the arm the gate's own sources happen to be written in.** A pair
covering one arm in four proves one branch and ships the rest proved by nothing,
and the failure is invisible: the uncovered arms still run on the live tree,
where they are green because the tree is clean. The rule is stated here because
the widening is the *parity instrument* rather than extra coverage — the pair is
the only oracle whose corpus is **inert under a port**. `gate-tests` is a member
of `GATE_PRUNE_DIRS` (§lib/gate.sh), so nothing inside a case dir is reachable
from a live-tree walk, nothing there is a registry member, and no port can add or
remove a file in one. A corpus carrying every arm that the port cannot move is
exactly what criterion 4 says a self-referential port must design and does not
say how to build (§The port-candidate criteria, criterion 4).

**A case dir is not its own git repository, and that decides what a git-walking
arm can see inside one.** A `git ls-files` run with cwd inside a case returns
the *outer* repo's index scoped to that subdirectory, printed relative to cwd.
Three consequences, none of them optional to design around: the `gate-tests`
prune never fires inside a case, because the scoping already stripped the prefix
it matches on; a **non-repository** arm is unreachable from any case dir and
cannot be exercised there at all; and a file a case plants to widen an arm is
invisible to the gate until it is `git add`ed, so an unstaged plant leaves the
widened arm running over nothing while the case still passes.

Two properties make the widening worth its cost rather than ceremony. Its
**planted violations are standing guards**: a `bad/` case whose only violation
lives in an arm reds forever if an edit later drops that arm from the walk — for
a gate-source auditor that is the difference between a caught regression and a
family of gates quietly printing `clean` over a corpus it stopped reading. And a
widened case tightens the crate-side read-set verification for free, because unit
test A observes each member running over its own cases (§Meta-gate conservation
for the binary substrate). The discipline that keeps the widening honest is the
existing one, applied per arm: **the named reader of every arm is the case's own
`expect.txt`** — an arm whose planted violation no `expect.txt` names has no
reader and is removed rather than added.

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
  `gate_path_pruned "$f"`. Compiled, that last adapter is `walk::path_pruned`,
  which sits beside `walk::prune_dirs` rather than once per consuming member: the
  predicate and the set it applies are one rule, so a per-member copy is a second
  implementation of it and the fifth consumer is what made that plain.
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

This section rules *whether* a gate is built; **what substrate it is built on**
is one step further and is ruled at §The port-candidate criteria, where new gates
are born native by default in a tree carrying the crate and shell is an exception
needing a stated cause.

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
already identifies the gate everywhere else. **The field roster is closed**, and
every field on it has a named reader — `# graph:` by the manifest readers below,
`# spec:` by canon-kit's `check-spec-pointer`, `# install:` by
§check-install-disposition (ruled at §The install disposition, *and in a `.gate`
descriptor on the same terms*), `# no-fixture:` by
§check-gate-fixture-coverage. The descriptor carries no field that lacks one,
reserving nothing against a future reader.

**Two fields are minted on shell, and the descriptor's roster is untouched by
either.** `# no-port: <cause>` declares that a script is never going to the binary
substrate and names the ruling that makes it so; `# port-until: <slug>` declares
that one is **still owed to the port and not takeable now**, and names the live
queue entry that owns the blocker. Each is one optional **header** line — in a
gate declaration, beside `# graph:`, `# install:` and `# spec:`.

**Their domain is any tracked script, not a gate declaration path, and that is the
same widening this section already applied on the other axis.** `# port-until:`'s
domain is ruled below as *any temporary hold with named work owed and a live owning
entry*, refusing to narrow to the born-native exception letters because the narrow
reading would leave the criteria section's own worked example unable to declare.
This is that sentence applied to the **corpus** rather than to the cause class, and
`# no-port:` widens with it — because a permanent shell disposition is exactly what
an install bootstrap has, and exactly what TRAJECTORY.md's completion predicate
asks a script to state. The enabling path is nothing but the file being tracked:
there is no registration step, which is the whole reason the field can reach a
corpus that owns no descriptor.

**Three things are explicitly unchanged, so the widening adds no second grammar.**
The payloads: `# no-port:` carries free text because permanence is a ruling whose
home is prose, `# port-until:` a bare slug, each for the reasons below. The
**mutual exclusion**: at most one of the pair on a file. And the closed-roster
rule: a `.gate` descriptor still carries **neither**.

**Its readers.** `# no-port:` is read by §port-blockers' `--group` arm over the
registry and its `--tree` arm over the tracked shell tree, and by
§check-gate-substrate-parity's assertion G. `# port-until:` is read by those three
and, for its slug's liveness, by §check-gate-exemption-tasks. Both are read by
§check-comment-tier, which is what keeps the line from reading as a restatement,
and which needed no widening: both spellings were already in its built-in directive
roster and its corpus was already the whole governed tree rather than the
declaration paths the fields widen *from*. **Assertions G and H do not widen with
the fields**, and stating that is the easily-missed half: their subject is the
*gate registry*, and a plain script has no registry membership and no `# spec:`
pointer for H to open, so widening their walk would red every script that declared.
The reader of a plain script's declaration is `--tree` and, for a slug,
§check-gate-exemption-tasks.

**What the widening does not gain is a knob**, and the reason it does not is worth
stating precisely, because the argument that held for the declaration path does not
reach the new corpus. Every reader over *declarations* resolves them through
`gate_resolve` under `gate_kit_roots`; `--tree`'s reader does not, resolving through
`git ls-files` and the prune-dir set instead. The conclusion survives on a different
ground: both are values `lib/gate.sh` already resolves for every pruned walk in the
tree, so the widened corpus rides configuration that predates it. **No exclusion
knob is minted either**, which §port-blockers rules and which is what makes the
owed count reaching zero *be* the completion predicate rather than approximate it.
**A `.gate` descriptor never carries either.** A descriptor's *existence is the
dispatch declaration*, so a member that has one is ported and has no port
question left to declare; such a line there would be a field asserting
the negation of the file it sits in, and the one way that lands in practice is a
port forward-copying it with the three lines that *are* copied verbatim — which
is precisely what assertion G's first clauses red on. The roster above therefore
gains nothing and reserves nothing.

**The held field's payload is a bare slug and nothing else**, which is the one
place the two fields' grammars part. A temporary hold has **two facts in two
homes**: the **ground** lives in the gate's own SPEC section, beside the rule it
governs, and the **disposition and the cost** live in the owning queue entry
(§The port-candidate criteria, which owns that split). `# no-port:` carries free
text because permanence is a *ruling* whose only home is prose and the field *is*
the pointer to it; `# port-until:` carries a slug because the slug reaches the
fact a *reader of the queue* needs, and the ground is already one hop away
through the declaration's own `# spec:` pointer. A `<cause>` half here would
therefore be a second place the same ground lives — the gate's section being the
first — and the one that rots when the entry is re-scoped. Both fields cost their
reader exactly one hop. The refusal is recorded so a later author does not
re-propose the half as an ergonomic, and the other hop has an oracle rather than
discipline behind it: §check-gate-substrate-parity's assertion H.

**A cause names the ruling it rests on, and that obligation is what the
permanence reading costs.** §port-blockers rules that `# no-port:` declares the
disposition **in force under the ruling that stands**, never an oracle about
future rulings — which leaves the field weaker than its name suggests to a first
reader and grows the cause's job to match. A cause that names its ruling lets a
later reader find out whether that ruling still stands; a cause that names none
leaves them nothing to check, and the field's own free-text payload is the only
place that pointer can live. The kit still constrains nothing but non-emptiness —
a cause is consumer content and nothing here parses one — so this is an authoring
rule the declaring unit holds itself to rather than an assertion, which is the
same disposition the free-text half already took when it declined the `# spec:`
pointer a machine could open.

**The slug's anti-rot is coarse, and the limit is stated rather than left to be
discovered.** Where several held members point at one umbrella entry — an entry
whose own text claims exactly that class of hold — every declaration reds at once
when the umbrella closes, and a member whose *own* blocker lands earlier keeps a
live-but-wrong declaration until then. That is strictly better than re-adjudicating
the holds by hand at every cut, and it is not the design's ceiling: **when a
blocker acquires its own designed answer**, that answer is a unit with an entry,
and the declaration re-points at it. Filing per-blocker entries speculatively
before then is costed filings against work nobody has scoped.

**Its domain is any temporary hold with named work owed and a live owning entry**,
which is wider than the two born-native exception classes §The port-candidate
criteria drafted it for. Those classes are statements about a gate being
*authored* in shell; the holders are not shaped that way, being criterion-7
blockers read over gates already authored and fitting no lettered class at all.
Narrowing the field to the letters would leave the criteria section's own worked
example unable to declare, which is the reading that fails on its own example.

**The `# reads:`/`# needs:` refusal below does not reach it, and the difference
is what a reader could verify.** Both were refused because nothing would hold
them to the implementation: they are claims about **runtime behavior**, which an
implementation can contradict, so the crate's registry plus a unit test that runs
the member is a strictly better home. These two are claims about a **design
ruling** and a **queue disposition**. Neither has a runtime referent at all — no
execution could falsify one — so *hold it to the implementation* names nothing
that could be done for it in any location. What `# no-port:` can be held to is the
ruling it cites, which the exception criterion already requires to exist in the
gate's own SPEC section (§The port-candidate criteria), and the pointer's shape is
what assertion G holds; what `# port-until:` can be held to is stronger, its slug
resolving against a live queue entry, which is why it does not rest on a shape
assertion alone.
**There is no value vocabulary on either**: presence *is* the verdict and absence
means takeable, so the payload is the whole of it — a reader who finds the line
reaches the argument, or the owning entry, in one hop.

**The kit ships the fields, their readers and the assertions — never a roster of
permanent or held members, never a cause text and never a slug.** A kit literal
naming one project's permanently-shell gates would publish that project's work
queue as everyone's mechanism, the defect §The first cohort, and the rule that
selects the next already guards against for the batch arm and the `check-graph`
rule-content split exists to prevent. The remainder is derived from declarations
in the **consumer's own tree** at every read, so a consumer with no permanent
member reads a field that never fires and one with ten declares ten, and a
consumer with no held member likewise. Both payloads are consumer content by
construction: a `<cause>` is free text pointing at whatever surface that consumer
records the ruling on, a `<slug>` names an entry in that consumer's own queue, and
the kit constrains only that each be non-empty — nothing here parses a cause,
matches it against a vocabulary, or knows what a section reference looks like.
No knob is minted for either, on the two grounds stated above — the declaration
readers resolve through `gate_resolve` under `gate_kit_roots` and the tree reader
through values `lib/gate.sh` already resolves — so the fields add no `<KIT>_<KNOB>`
and no default to be unset anywhere.

**On a plain script only the held field earns a liveness reader, and the asymmetry
is deliberate.** A stale `# port-until:`, whose blocker landed and whose slug moved
to Done, **under-counts** the owed set and hides real work — the direction no shape
assertion covers — so the slug is held to a live queue entry wherever it sits. A
`# no-port:` cause gets no such reader: it is free text pointing at whatever surface
records the ruling, and what holds it honest is review at the diff. For a *gate* the
second holder is assertion H, which opens the section the declaration's own `# spec:`
names — an assertion a plain script cannot satisfy, having no such pointer. **The gap
is named here rather than closed**, because the cheap closure would demand a
`# spec:` pointer on every declaring script, minting a second obligation to buy an
assertion the free-text field was chosen not to need.

**Two fields are refused rather than merely absent, and the refusal is the
design.** A `# reads:` line declaring the gate's walk roots is the obvious cheap
way to answer §check-reads-couples, and it is rejected: nothing would hold it to
the implementation, so it is the removed `# reads-couples-exempt:` opt-out with
more words — a self-declaration whose would-be reader could not verify what it
read. A `# needs:` line declaring the member's external programs (§The
port-candidate criteria, criterion 7) earns the identical refusal on the identical
ground, and is recorded here so a later author does not re-propose it as the cheap
answer to §port-blockers' undecidable count. Both sets live in the crate's
registry instead, where a unit test runs the member and compares the declaration
against what it actually did. This is
also why §check-gate-substrate-parity assertion D's manifest-class partition is
left unchanged deliberately: a roots declaration is *implementation data*, not
manifest-class, so it belongs in the implementation and not in the build-free
surface.

**The interface that answers is substrate-neutral; its placement is not settled
here.** The contract is *a declaration path's substrate answers what it reads* —
shell answers by parse, the binary answers by `--reads`. It generalizes to
requirements the same way: **shell answers by parse, the binary answers by
`--needs`** — a top-level flag beside `--list` / `--reads` / `--knobs` /
`--source-stamp`, backed by its own registry-tuple element and held to behavior by
a crate unit test in the shape of the `--reads` one. **The arm is built**, on the
condition this section set for it: it was sequenced rather than shipped while no
ported member required an external program, because it would have landed with no
named reader, and the first port of a member carrying one builds it. No ordinal is
stated for either the flag or the element, deliberately: both rosters churn with
every port, and a count in prose over a churning roster is a second source for
something `main.rs` and `GateEntry` already hold.

**Its report grammar is one line per requirement and nothing else** — no count
line and no header, on §check-reads-couples' ground that a transcribed total is a
second source for something derivable from the lines. Three line kinds, each with
a named reader at a named transition:

- **`<program>`** — a program the member spawns. Read by §port-blockers' default
  arm at its per-member row, filtered against `GATE_SDK_PROGRAM_FLOOR` exactly as
  a shell member's scanned command word is, so an on-floor program is suppressed
  on both substrates by one rule.
- **`?<TAB><knob-name>`** — the requirement is the **command word of that knob's
  resolved value**. Read by the same arm at the bridge resolution that precedes
  the floor filter, the path it already uses for a shell member's command-position
  expansion, so the two substrates cannot disagree about a knob's value. This is
  the load-bearing kind rather than an ergonomic: no literal roster is true for
  every consumer (§The port-candidate criteria, criterion 7), so spelling the
  program into the registry would be a second copy of the knob's default and
  *wrong* for any consumer who repointed it — silently, since nothing would
  compare the two.
- **`?`** — a requirement the registry cannot bound at all. Read by the trailer's
  undecidable counter, the reader a shell member's unresolvable expansion has.

**The `?` in the knob form is deliberate reuse and not an inconsistency with
`--reads`, which spells its optional field the other way round.** There a root is
an answer and the tab field refines it; here `?` says *the registry cannot name
this literal* — what `?` already means on both arms — and the tab field says
*where the answer is instead*. A reader who takes the tab field away is left with
the honest `?` the arm would otherwise print. Stated because the two grammars are
read side by side and the asymmetry would otherwise look like drift; it is also
why one covering rule holds both arms to behavior rather than two.

**Unit test A holds the declaration to executed behavior**: every registry member
runs over its own `gate-tests/<name>/{good,bad}/` cases with the spawn recorder
armed, and the observed program set must be a **subset** of the declared one. The
direction is stated so a later reader does not tighten it to equality: a wrapper
whose program is reached only on a branch no fixture case takes would fail an
equality test for being correctly declared, and the failure that matters is an
**undeclared** spawn — over-count rather than lose, the direction criterion 7
already fixes for an undeclared hold. **The reach of that guarantee is bounded by
the fixture corpus, and the bound is stated rather than left to be discovered.** A
case runs one argv, so it selects one mode; a member none of whose cases selects a
mode that reaches a spawn is observed spawning **nothing**, and the subset
assertion is vacuous over it — the undeclared spawn A exists to catch is caught
only where a case reaches the branch that makes it. `check-graph` is the live
instance (§gen-pre-commit), and it is the shape to look for: a member whose
fixture pair is deliberately narrowed to one hermetic assertion. What holds such a
member is whatever behavioural driver reaches the branch, which is a different
oracle and not this one. Test **B** — no `Command` construction
outside the crate's one sanctioned spawn wrapper — is what makes A trustworthy,
and it is already enforced by `proc.rs`'s own unit tests rather than added here.
A member whose registry entry omits the element fails to compile, on `--reads`'
terms. Whether a later authoring
SDK relocates that contract to a substrate-neutral surface stays
`gate-authoring-sdk-surface`'s question, narrowed by this and deliberately not
answered: one substrate's answer is added without adding a shape that would have to
be unbuilt to generalize.

**The descriptor is a durable surface, not port scaffolding, and it is in live
use.** The first cohort's two members declare this way — `check-action-pinning`
and `check-action-gh-repo` (§The first cohort, and the rule that selects the
next) — so the format below is specified against shipping files rather than
against an intended one. Its reason does
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
  queue-tracked marker; `none` means the sides must agree now. **It is
  `check-graph`'s field and nothing else's**, and in particular it has no
  relation to a gate's own exemption marker: a gate that owns an exemption
  array still declares `valve=none` here, and `check-gate-exemption-tasks`
  reads the exemption arrays and `# port-until:` headers rather than this
  field. Stated because the two are read as one classification otherwise, and
  the descriptor gives no other clue that they are separate.
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

## The install disposition

A gate declares **its own install disposition**, one `# install: <disposition>`
line in its header block beside `# graph:` and `# spec:`, and in a `.gate`
descriptor on the same terms — a ported gate is still a gate a kit ships, and
the descriptor rides the payload like any other member (§Consumer payload). The
vocabulary is closed, three values:

- **`zero-config`** — the gate reads a surface `init` itself writes, so it
  registers in a fresh consumer.
- **`on-surface`** — the gate's subject is one the adopter authors later (a
  glossary, a docs host, a stage attestation, their own workflows), so it arms
  when that surface exists rather than at install.
- **`never`** — the gate is not auto-registered on any tree, because its subject
  cannot exist in a vendored tree at all or because it is declared
  never-registered. This is the class §Consumer smoke's declaration valve
  already recognises from the other side.

The vocabulary names **install-time reachability**, never a path, a surface
name, or a project's own governed-file vocabulary. That is deliberate: a
disposition enumerating consumer paths would be a kit literal carrying one
project's tree layout, which no kit may ship. The path a gate reads stays where
it already lives — the gate's own configured knob.

**The installer's per-kit roster is derived from these declarations, not
maintained.** `recipe_gates` in the installer's `lib/common/recipe.sh`
(installer/README.md §What init seeds) is every `checks/` member of a kit — both
declaration spellings — whose disposition is `zero-config`. It carries no gate
name of its own, which is what `check-install-disposition` assertion C holds.

**Two rosters, two trees, and that is why the disposition is kit-owned rather
than the roster.** A kit's `smoke/install.sh` registers against a scratch
consumer *the smoke script itself builds and seeds*; `recipe_gates` registers
against the tree `init` makes. The installer's set is a subset of the smoke's in
every gated kit, and the difference is deliberate where it is largest:
lifecycle-kit registers nothing at install because its gates read a stage
attestation only a stage session can write, while its smoke stamps that
attestation for real; site-kit registers nothing at install and five in smoke
because the smoke writes `docs/CNAME` and a workflow template `init` never
writes. A single entry point returning one roster could serve neither caller
without arming gates on a tree that cannot satisfy them or stripping the smoke's
coverage. What is kit-owned is therefore the **disposition**; each caller derives
its own set from the tree it actually made.

The exposure this closes is that a kit adding a zero-config gate the installer
never learned about shipped to adopters unregistered and silent. A gate cannot
now reach an adopter undeclared, because the declaration is enforced present.

**Where the directive deliberately does not reach.** The installer's *seeding*
decisions stay keyed by kit, and they look like the same defect this section
removed. They are not: they answer what a kit's install must **seed**, not what
it may **register**, and a seeded surface is a property of the kit rather than of
any one gate — so a per-gate directive has nothing to say about it. Which
decisions those are, and how each is made, is the installer's
(installer/README.md §What init seeds). Recorded so the boundary is found drawn
rather than extended into a place it does not fit.

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
a gate, and matching it would over-report.) **Where a declaration lives is not a
term of that derivation**, on either side of it: a member the consumer's own
gates directory declares earns a row on exactly the same terms as a kit-declared
one, and a consumer-declared member's own declaration path counts as an object
the couples may cover. Said in a sentence because the table below is entirely kit
gates and the natural reading of that is a false exemption — an exemption whose
cost is the silent end of whatever assertions a consumer-declared meta-gate
makes, the one failure mode this section exists to prevent. A tranche porting
consumer-declared members re-derives the set at its cut rather than inheriting a
verdict, because a port *changes* a declaration path — `<gates-dir>/<name>.sh`
becoming `<gates-dir>/<name>.gate` — which can move other members into or out of
the derived set. Every derived member takes exactly
one recorded disposition below, and a member the section does not name is red.

**Re-taken at `shell-gate-tail-port`'s cut, from clean checkouts of both sides:
the derived set falls from 29 members to 9, twenty leave it and none enters.**
The largest single movement this table has recorded, and the cause is one thing
rather than twenty: every departing member was selected by a `*.sh`-shaped
token — `kit:*.sh`, `scripts/*.sh`, `gate-sdk/checks/*.sh`, `kit:*/*.sh` — and
that unit emptied the tree of shell gate declarations, so **no `*.sh` glob covers
a declaration path any more** and the whole class drops out at once. The nine that
remain are exactly the members whose couples name the `.gate` spelling or a
spelling-blind directory glob: `check-comment-tier`, `check-deprecation-task`,
`check-spec-pointer` and `check-todo-task-liveness` through `*.gate`;
`check-gate-assertions` and `check-gate-binary-fresh` through
`gate-sdk/checks/*.gate`; `check-install-disposition` through
`gate-sdk/checks/check-*.gate`; `check-gate-substrate-parity` and
`check-readme-roster` through `gate-sdk/checks/*`. Each holds a row below.

**What that costs, stated because no row carries it and no assertion can.**
Assertion C is one-directional — it reds on a derived member with no disposition,
never on a disposition whose member the derivation stops selecting — so a set that *shrinks*
passes green the whole way down, which is how a movement of twenty arrives with a
clean battery over it. The consequence outlives this cut: `kit:*.sh` was the
commonest route into the derived set and is now a route to nothing, so a
**later meta-gate reaching for that spelling receives no assertion-C coverage at
all** and its author gets no signal saying so. The derivation is still correct —
it asks what a member's couples cover, and they cover no declaration — but the
protection it buys is now concentrated on nine members and on the `.gate` spelling
alone. Whether the derivation should widen is a question this unit does not
answer and files rather than takes; what a reader must not do is read the table's
length as the set's size.

**`check-crate-arms` couples the crate and is still outside that derivation**,
stated here so its absence from the table reads as a verdict rather than an
omission. The test is whether a member's expanded `couples=` covers a registry
member's **declaration path**; `native/src/gates/*.rs` is where a ported gate's
rule is *implemented*, never where one is declared, so the derivation does not
select it and no disposition row is owed. A later reader adding one would be
answering a question assertion C never asked. **It is itself `.gate`-dispatched
since `shell-gate-tail-port`, and its own port does not move it into the
derivation** — re-taken at that port rather than assumed, because a port changes a
declaration path and that is exactly what can move a member across this line. What
its couples name did not change and is still implementation, so the verdict above
holds for the same reason it always did.

| Meta-gate | Disposition for a `.gate`-dispatched member |
|---|---|
| `check-shellcheck` | **Retired with cause, and the cause is per-member rather than about this gate.** For a `.gate`-dispatched member there is no shell file to lint, so this meta-gate makes no assertion about it; `cargo clippy` at deny-warnings is the substrate equivalent and runs in CI, not as a gate. Read as a statement about the *gate* the row would be false, and the distinction is worth the sentence: the gate is `zero-config`, an adopter cannot author a compiled gate, and a vendoring consumer's gate family is shell by construction — so what ends when a tree's last `.sh` leaves is that tree's registration, never the shipped gate (§check-shellcheck). **This member is itself `.gate`-dispatched since `shell-gate-tail-port`**, ported as criterion 7's wrapper: its rule is an invocation of `shellcheck`, which stays a declared dependency the compiled form spawns and refuses at exit 2 without. Its own port moves nothing in the rule and one thing in its corpus — one fewer `.sh` to lint — which is this row's disposition measured rather than asserted. |
| `check-gate-output` | **Ported and strengthened for the fixtured corpus; source-grep retained for the one member outside it, over the corpus that member's rule now lives in.** The source-grep for `: clean`/`help:` was always a proxy for behavior; for the fixtured members the assertion now runs in `run-gate-tests.sh` (§run-gate-tests) against the case's real output, on **shell gates too**. The remaining member, `check-task-conservation` (`# no-fixture:` per queue-kit/SPEC.md §check-task-conservation — a HEAD-vs-worktree diff has no static-fixture representation), has no case for a runtime assertion to reach, so the source-grep stays its only oracle. Retiring the static half outright would zero out that member's output-contract coverage — the exact vacuity this table exists to close. **That member has since ported**, which is why this row is not "unchanged": its declaration path is now a descriptor, which by the closed field roster cannot hold the strings, so corpus *and* emitter alternation follow the rule to the implementation module, and a tree carrying no crate declares the member out of reach rather than reddening (§check-gate-output owns the resolution and its two branches). |
| `check-gate-fail-closed` | **Retired with cause, and the cause is narrower than it first read.** For a member that reads files, the defect (branching on a captured value's emptiness when the subprocess died) is unrepresentable: there is no subprocess, and a fallible read returns a `Result` that cannot be ignored. A real substrate win, stated as one. **It is representable for a member that spawns one**, and the queue-kit cohort landed the first: `Command::output()` returning `Ok` means the *spawn* succeeded, never that the program did, so reading `stdout` while ignoring `status` reproduces the defect exactly. The disposition is unchanged — this gate's corpus is `check-*.sh` and it could not scan a Rust module either way — and the property is held crate-side rather than by review: the spawn wrapper and its unit tests (§Fail-closed contract) leave a gate module unable to construct a `Command` at all, and unable to reach stdout without the status having been read. Machine-held rather than remembered, which is the same answer the `check-reads-couples` row below gives to the same problem, and what keeps this retirement honest. |
| `check-reads-couples` | **Retained, with a binary-side equivalent.** Its shell parser finds no walks in a binary gate and would print `clean` — the single worst vacuity available here — so the substrate answers instead of the parser: the binary carries a `--reads <name>` arm printing one line per walk root, a repo-relative path or `?`, and the gate consumes that report into its existing coverage assertion (§check-reads-couples). The declaration is **registry data held to executed behavior**, which is what separates it from the unbound self-declaration this gate exists to refuse: each gate's roots are declared beside its dispatch entry in the crate's registry (an entry added without them fails to compile), the crate's single sanctioned walk implementation records the roots it is invoked with, and two unit tests close the loop — **A**, every member run over its own `gate-tests/<name>/{good,bad}/` cases with recording on, observed roots a subset of declared; **B**, no module outside that walk implementation names a filesystem-walk API, because a direct walk would be invisible to the recorder and unverify A. B's vendored half is held by an **allowlist over the resolved graph**: a spelling roster cannot catch a walker inside a dependency, so every crate in the tracked `Cargo.lock` — transitive included, since a transitive crate walks as visibly as a direct one — is admitted by name with the clause of the dependency bar it cleared (§The settings cohort, and the crate's first dependency), and the assertion reds both on an unadmitted crate and on an allowlist entry absent from the graph. Reading only the `[dependencies]` table would admit an entire subtree unexamined, which is why the lock is tracked rather than gitignored. The precedent is the `check-knob-default-coupling` row below: an executed assertion is the answer where a static gate would be vacuous. The refusal survives only where the gate still cannot see — a name the substrate does not carry, and an unresolvable filter knob — and there is deliberately no descriptor-level opt-out, which the consumption path does not reinstate: a port ends this assertion by answering it (§check-reads-couples). **This member is itself `.gate`-dispatched** since §The sixth budget batch, which is that closing clause discharging on the auditor: the compiled form reaches the read set in process rather than spawning the arm, so the absent-binary refusal is answered out of existence rather than retired, and the row now describes a ported member auditing ported members. What `--reads` verifies is unchanged and is worth restating because the natural reading is wrong: a member's declared roots are **registry data**, not a derivation from its Rust source, and the declaration-to-code link is held by unit test A. Both members ported in that batch with a non-empty root set carry `?` alone, and the auditor's own root set is empty and stays empty, so no self-assertion is lost. |
| `check-gate-assertions` | **Retained, corpus extended** to the gate's Rust module; the `# assertion` marker matches on its token, independent of the comment leader. **This member is itself `.gate`-dispatched** since the eighth budget batch (§The first cohort, and the rule that selects the next), and its port moved more than its own spelling: the section it audits gained an enumerated contract of its own, so the gate now reads **its own implementation module** and the contingent immunity that kept its own heading out of discovery is ended deliberately (§check-gate-assertions). Its fixture pair, not the live tree, is what proves those arms — the `check-comment-tier` sentence, inherited for the same reason. |
| `check-gate-exemption-tasks` | **Retained, corpus extended** the same way. **This member is itself `.gate`-dispatched** since §The sixth budget batch, so the row describes a ported member reading ported members' declaration paths — and its own port changed nothing in the rule: what it globs is both declaration spellings, which a descriptor still is. |
| `check-comment-tier` | **Retained, corpus extended** to the implementation module and the `.gate` descriptor, whose own lines are directives by construction. Mechanism: the shared primitive `comment_surface` carries `*.gate` **and `*.rs`** arms — widened once, for every caller (see the `check-spec-pointer` row). The implementation arm is the load-bearing one: locality-class directives stay in the implementation by the reader partition (§The `# graph:` manifest), so without it they would go dark exactly where they still apply. **This member is itself `.gate`-dispatched** since the seventh cohort, so it now audits its own declaration — which is why its trigger names `*.gate` and `*.rs` and why its fixture pair, not the live tree, is what proves those arms. |
| `check-spec-pointer` | **Retained, and its corpus depends on the same widening** — not "unchanged" in mechanism, only in assertion logic. It calls the *same* shared primitive, `comment_surface` in `native/src/spec.rs` since the seventh cohort ported both it and all four of its callers (canon-kit/SPEC.md §lib/spec.sh); absent that one shared fix a ported gate's `# spec:` line would silently stop being checked in both places it can live — the descriptor and the implementation. With the primitive carrying the `.gate` and `*.rs` arms its own probe logic needs no change. |
| `check-readme-roster` | **Retained, glob widened** to `*.sh` + `*.gate`. Without it a ported gate silently drops out of its kit README's roster in both directions. **This member is itself `.gate`-dispatched** since §The second budget batch, so the row now describes a ported member reading ported members' declaration paths — the shape the `check-value-rollup-fresh` row already has, and the port changes nothing about the rule: what it scans is a set of basenames across both spellings, which the descriptor still is. |
| `check-exec-bit` | **Retained, extended**: a `.gate` descriptor must be **non**-executable. Stated as an assertion so "not executable" cannot read as "not covered". |
| `check-todo-task-liveness`, `check-deprecation-task` | **Retained, corpus extended** to the Rust module and the descriptor, the same shape as `check-comment-tier`: both walk the shared comment surface hunting `TODO(task:)`/deprecation markers, so a marker left in a ported gate's Rust source would otherwise stop being tracked. Both are `.gate`-dispatched since the seventh cohort, on the same terms as the two rows above. |
| `check-knob-default-coupling` | **Retained unchanged, and deliberately *not* corpus-extended** — the extension the shape of this table invites would be vacuous. Its two default idioms are shell (`${KNOB:-v}`, the guarded assignment) and its knob prefixes derive from `gate_kit_roots` members; `native/` is not a kit root and a Rust `const` matches neither idiom, so pointing it at `*.rs` would scan files whose grammar it cannot parse and add zero assertions while reading as coverage. The duplication it could not reach — the crate's prune-dir default against `lib/gate.sh`'s — **is now absent rather than test-held**: the config bridge (§lib/gate.sh) leaves exactly one place a knob's value is computed, the kit's shell library, and the crate carries no default for a bridged knob to drift from. The crate carries no unit test comparing the two literals, because it carries only one: that assertion is **deleted with the duplication it gated** — enforcement-first ranks removing the duplication above gating it, and a citation left behind would point this table at an absent mechanism, the exact defect its own prose calls out. Its verdict on `lib/gate.sh` is unchanged: the shell default stays exactly where it is, as the sole one. **This member is itself `.gate`-dispatched** since §The sixth budget batch, so the row now describes a ported member — and the port moves nothing in it: the two idioms it scans are shell whichever substrate reads them, so the not-corpus-extended verdict above is a property of its rule rather than of its own declaration's spelling. |
| `check-gate-tamper` | **Retained, extended — and the extension is partly discharged.** The gate-file roster it recognises (`DELEGATION_KIT_GATE_FILES`) carries the `.gate` spelling on the **kit default** as of the first cohort's descriptors, or a consumer on that default would receive a ported gate whose edits escape the isolation rule (delegation-kit/SPEC.md §Layout and configuration). This repo's own config carried both spellings ahead of the port, which is exactly why the kit default had to be checked separately rather than inferred from a green battery here. Its meta-layer path roster (`DELEGATION_KIT_META_PATHS`) is fixed **in this repo's consumer config**: `native/` is declared there (`scripts/delegation-config.sh`), so a commit editing a ported gate's Rust implementation alongside its descriptor is meta-isolated rather than refused. The kit default and any other consumer on it still lack the prefix — `native/` is never auto-unioned by the kit-root scan (`gate_kit_roots`'s predicate requires `checks/` or `smoke/`, which the crate ships neither of), so the fix is consumer config, not a kit-default change. One known limit stands: its exemption reader parses a shell `# exception-list:` array literal and has no Rust-source equivalent. Stated against the live ported set rather than the cohort that first raised it, since the binding condition is a property of the roster and moves with every cohort: the limit is unbound while no ported member carries an exemption list, and it is unbound for a stronger reason than a shell-only holder set — the tree carries **no** exemption-list holder at all, which `check-gate-exemption-tasks` counts on every run. The cohort that ports the first one owes the Rust-source reader. |
| `check-graph`, `check-kit-enum`, `check-gate-fixture-coverage`, `check-enforcement-fresh` | **Survive unchanged** — all four read the declaration path as text (directly, or through the enforcement-map and footprint emitters, which do), which the descriptor still is. |
| `check-value-rollup-fresh` | **Survives unchanged in mechanism, and is itself `.gate`-dispatched** since §The consumer remainder cohort — so this row now describes a ported member reading ported members' declaration paths. It reads them as text through the footprint emitter, a non-gate arm this gate calls in-process (§The non-gate arm), and the declaration path is what that emitter reads — which is why the port moves nothing about its rule. What the port *did* move is one term of its coupling, recorded because the re-derivation confirmed it rather than assuming it: its `couples=` names `scripts/*.sh,kit:*.sh`, and after that cohort emptied the consumer's gates directory of check scripts, `scripts/*.sh` covers **no** registry member's declaration path at all. That left `kit:*.sh` as the whole of its selection, and **`shell-gate-tail-port` emptied that token too**: with no shell gate declaration left in any kit, the derivation does not select this member and its row is a **retained record rather than an owed disposition**. Both halves of the same narrowing, a cohort apart, which is why the second is stated here rather than treated as new. A narrowing is still not a clearance — `scripts/` and the kits retain many non-gate `*.sh`, and the coupling still earns its trigger — but a later reader deriving the set must not read either `*.sh` token as the thing that selects this row, because neither does. |
| `check-gate-binary-fresh` | **Retained by construction — and recorded here before the derivation reaches it, deliberately.** It reads declaration paths as a *set*, to decide whether the binary is load-bearing, and never reads a gate's source, so a port is its trigger rather than its blind spot: a ported member is exactly the case that switches it on. Its couples name `kit:checks/*.gate` specifically, so it was **not yet substrate-sensitive** by assertion C's runtime derivation when this row was written, with zero descriptors then on disk, and the row was not yet owed — it was written ahead of the trigger rather than left to be discovered. The first cohort's descriptors have since landed, so the gate is sensitive and the row is owed; the commit that landed them would have reddened on a missing disposition, and that commit's session was the worst possible one to be learning this table exists. That is the foresight paying, and it is the same reasoning as the gate itself: the oracle ahead of the hole (§check-gate-binary-fresh). |
| `check-gate-substrate-parity` | **Retained by construction, and `.gate`-dispatched since `shell-gate-tail-port`** — it is substrate-sensitive by the same derivation it performs, and it reads declaration paths both as text and as a *set*, which is precisely what it exists to see, so this row now describes the auditor of the dispatch relation auditing itself. It ported under the 2026-08-23 ruling that retired born-native exception class (a): the shell form already read one side of its comparison through `--list`, so the auditor's independence from the binary was never more than the absent-binary case, which the fail-closed contract owns — and compiled, that case has no reachable input, the binary being the process the assertion runs in. The port moves nothing in the rule: the descriptor set is still globbed off the resolve dirs and the roster is still the binary's, reached in process rather than through a spawn (§check-gate-substrate-parity). Its own row is written out rather than left to the section's prose mention: assertion C is satisfied by any occurrence of a member's name in this section, and a gate passing its own assertion by being *discussed* is a coincidence, not a disposition. |
| `check-install-disposition` | **Retained, and substrate-blind by construction** — it reads both declaration spellings as text, taking the `# install:` header line off a `.gate` descriptor exactly as off a `.sh` implementation, because a ported gate is still a gate a kit ships and its disposition is a property of the gate rather than of its substrate (§The install disposition). A port therefore moves nothing here: the declaration travels with the descriptor, which is the same file the installer's payload already carries. It is **`.gate`-dispatched since `shell-gate-tail-port`**, ported with its sibling auditor under the 2026-08-23 ruling: the assertion that a gate declares itself is a text walk over both declaration spellings, and a binary that is absent cannot pass it silently — the battery exits 2 rather than skipping (§Fail-closed contract). What its own port moved is one number rather than any part of its rule — its clean line counts one fewer `.sh` and one more `.gate`, which is the substrate-blindness above measured rather than asserted. |
| `check-docs-cmd`, `check-install-claim`, `check-payload-claim`, `check-queue-slug-liveness` | **Survive unchanged — reverse triggers.** Each names `scripts/*.sh`/`kit:*.sh` in `couples=` only so that a script change re-runs it; the corpus each actually scans is the governed-doc set, and none reads a gate script's *content* as its assertion target. `check-docs-cmd` is worth naming: it will correctly — not vacuously — red on a doc still fencing a deleted `.sh` path after a port. That is real signal. Every member of this row is a ported one — `check-queue-slug-liveness` since the queue-kit cohort, `check-docs-cmd` since the canon-kit one, and the remaining pair since the ERE cohort — so the row describes `.gate`-declared gates throughout; the reasoning is unaffected, because what they scan is the governed-doc set rather than any gate's content. |
| `check-settings-paths` | **Survives unchanged — reverse trigger, and a port is its subject rather than its blind spot.** Its `couples=` names `kit:checks/*.sh` only so that a check-script edit re-runs it; what it scans is the committed permission allow-list, never a gate script's content. A port is the event it exists for: replacing `checks/<gate>.sh` with a descriptor strands every allow entry naming the old path, so the gate reddens *because* of a port rather than falling silent after one — the shape `check-docs-cmd` has in the row above. Two limits are recorded rather than left to be re-derived. The glob is deliberately not widened to `*.gate`, because a descriptor path is not something a `Bash(…)` grant invokes and the widening would add no assertion. And the trigger is a **partial route by construction**: the generated hook matches staged `ACMR` paths, so a *deleted* `.sh` never fires it; what catches a cohort's stranded grants is the whole-tree battery, which runs with no trigger filter. The trigger still earns its place — it catches the ordinary edit that strands a grant — but it is not what makes the gate's landing order necessary (context-kit/SPEC.md §check-settings-paths). **This member is itself `.gate`-dispatched from the settings cohort**, so the row describes a ported gate: the reverse trigger and both limits above are properties of its rule, not of its substrate, and survived the port unchanged. |
| `check-prose-enum` | **Corpus extended to the Rust module — it was never a pure reverse trigger.** This gate was grouped with the reverse triggers above on the ground that none of them reads a gate's *content*; that ground was **false for this one**, and the queue-kit port is what exposed it. Its enum derivation (`scripts/enum-sets.sh`) reads the queue tag vocabulary out of `check-tag-lead-line`'s own class table, deliberately — *"read from the gate rather than re-listed here, so a rename cannot leave the two spellings disagreeing"* — so deleting that gate's script broke the derivation and the gate exited 2 rather than passing vacuously, which is the fail-closed behavior working. The corpus follows the rule to where it now lives, `native/src/gates/tag_lead_line.rs`'s `CLASSES` table, keeping the read-from-the-owner property and its one-table fail-closed anchor. **The gate is itself a ported member since the canon-kit cohort**, so a gate whose input is a gate's content is now gate content — and its own derivation crosses the bridge as *data*, which is what keeps the compiled form from spawning the emitter it reads. |
| `check-measured-claim` | **Retained, and sensitive through its oracle rather than its corpus — the first born-native member to take a row.** What it scans is the governed-prose surface, so by corpus it is a reverse trigger like the row above. What made it substrate-sensitive was the consumer oracle behind `CANON_KIT_MEASURED_CLAIMS_CMD`, which its `couples=` reached through `scripts/*.sh`: this repo's emitter counts how much of the registry resolves to a `.gate` descriptor, so it reads declaration paths **as a set**, the shape `check-gate-binary-fresh` has. `shell-gate-tail-port` emptied that token of declarations, so the derivation does not select the member and this row is a retained record — but the reading below is what the row was for and it is untouched by that, which is the distinction to carry: **selection by the derivation and sensitivity in fact are two different questions**, and this member is the case where they came apart. A port still *moves its value*, which is the mechanism working rather than a blind spot — the number a marked sentence states is about the port, and the sentence reddens when the port advances without it. The design that landed the gate predicted no row here, on the premise that its `couples=` named no declaration path; the emitter coupling the same design requires falsifies that premise, and the row is recorded rather than the coupling dropped, because dropping it would leave the oracle's own source outside the trigger set. **The row and criterion 4 are independent facts, and this is the case that proved it**: the criterion binds on a gate's assertion target, this gate's is the governed-prose surface, so it clears — while the transitive reach through its emitter is precisely what assertion C is shaped to see (§The port-candidate criteria, criterion 4). |
| `check-unmarked-claim` | **Retained, and sensitive by coupling rather than by corpus or by oracle — the narrower case beside the row above, recorded so the two are not read as one.** Its assertion target is the governed-prose surface, so by corpus it is a reverse trigger. Its own consumer roster (`CANON_KIT_CLAIM_CLASSES_CMD`) reads no declaration path at all — it emits a fixed vocabulary — so unlike its sibling above, a port does **not** move its value. What made it substrate-sensitive was `kit:*.sh` in its `couples=`, which reached the kit library resolving its knob and the shell members beside it; the coupling is correct and stays, because the library that runs its emitter must re-trigger it. Two consequences a later reader would otherwise re-derive. The trigger **narrowed to nothing on this axis when the residue emptied**, and that is measured rather than predicted: `shell-gate-tail-port` took the last kit declaration in that spelling, so the member **is** now a pure reverse trigger and this row is a retained record rather than an owed disposition — the same narrowing recorded for `check-value-rollup-fresh`, arriving in the same cut and for the same reason. And its composition partner is where the port actually lands — a consumer whose class is about the port marks the sentence with that gate's oracle key, so the moving value lives in `check-measured-claim`'s row and never in this one, which is exactly why coverage-only (canon-kit/SPEC.md §check-unmarked-claim) costs nothing here. |
| `check-spec-embedded-source` | **Survives unchanged — reverse trigger of the same shape.** Its `couples=` extension list (`*.rs`, `*.sh`, `*.toml`, …) is the roster of **languages it recognizes inside fenced blocks**, not a reference to gate declarations; its scanned corpus is the canonical specs and amendments. It already carries `*.rs`, so a ported gate's Rust module is inside its trigger set with no widening. **This member is itself `.gate`-dispatched** since §The sixth budget batch, so the row describes a ported member — and the reverse-trigger reading survives its own port, because the extension list is still a language roster and not a declaration reference. What its port *does* move is its own candidate index, which loses a shell declaration and gains a Rust module at every sibling's port; that is a property of the corpus rather than of this table's question (canon-kit/SPEC.md §check-spec-embedded-source). |
| `check-template-copy-parity`, `check-template-registry-parity` | **Survive unchanged** — their corpus is kit templates and the template registry, not gate declarations; a gate's substrate does not reach either. Both are `.gate`-dispatched — the second since §The consumer remainder cohort, the first since §The sixth budget batch — so the row describes ported members, and the reasoning is unaffected for the reason it gives: neither reads a gate declaration at all. |

**No gate stays shell because it audits the dispatch relation — ruled 2026-08-23
by the operator, retiring the rule this paragraph used to state.** Two rows above
were held on shell under it — `check-gate-substrate-parity` and
`check-install-disposition` — on the ground that a compiled auditor of the
declaration and dispatch relation could pass *itself* with a broken binary, a
**false green**. The ground does not survive inspection, and the refusal is
recorded with it so it is not re-proposed. The shell parity gate already takes
`--list` from the binary as one side of its comparison, so its independence was
never from the binary's *answers*, only from its *presence*; and presence is
owned elsewhere — an absent or non-executable binary is exit 2 under §Fail-closed
contract, never a skip, and a stale one is §check-gate-binary-fresh's red. What a
compiled auditor adds to that is nothing the shell form did not already trust.
The rule is therefore: **every gate ports**, an auditor of the dispatch relation
included; a gate that reads declaration paths as content pays criterion 4's price
for the parity oracle (§The port-candidate criteria). The residual the old
distinction named is unchanged and answered the same way — after such a port a
corpus-walk regression silences those gates over the crate's own sources with no
shell auditor left to see it, and the answer is the widened `bad/` fixture cases,
which red on exactly that edit and which a consumer receives and can run without
the crate (§Fixture-pair discipline). What survives of the old rule is its
*scoping* lesson: a selector applying "audits gate source" mechanically would have
held the whole `spec_comment_surface` family, where the ruling never did.

**The declared read set's honest limit, stated rather than discovered.** Unit test
A's coverage is the fixture corpus. That is bounded by a contract rather than by
luck — every ported gate carries a `good/`+`bad/` pair (§Fixture-pair discipline),
enforced by §check-gate-fixture-coverage — but a walk reachable only on an input no
case exercises is observed by nothing, and the declaration for it rests on the
author. The `?` line exists for exactly that case: a gate whose author cannot bound
a root declares `?` rather than guessing, and the reader counts it as undecidable
instead of trusting it as empty. Test A holds a `?` to its arity, not to nothing —
each one absorbs a single unmatched observed root, so a second undeclared walk still
reds.

**Where that verification runs, and where it does not.** Those unit tests are
`cargo test` — and so is the third registry-data test,
`every_registry_member_declares_the_root_that_carries_its_descriptor`, which
holds each member's declared root to the descriptor that root actually carries,
over both root shapes (§check-gate-substrate-parity, assertion B's owner column);
it is named *root* rather than *kit* because a test whose name says one shape
while it asserts over two is a name that will be read instead of the body. For
the sentinel it must know the consumer's gates directory, and it asks
`gate_sdk_gates_dir` for it rather than carrying a copy of that layout: the crate
holds no default for a knob the kit's shell library resolves, the same rule the
`check-knob-default-coupling` row above states, and a gate **module** wanting the
same value crosses the config bridge instead (§lib/gate.sh). Learning a layout
inside a `cargo test` is sound **here and nowhere else**, and the bound is the
next sentence rather than taste — these tests run
in this repo and in CI and **never in a consumer tree**:
`native/` ships no `checks/` and no `smoke/`, so it is not a kit root and no
consumer ever receives the crate source — there is nothing there to run them
against and nothing there to edit. That is not a weakening; it makes the division
explicit. The declaration is held to executed behavior **upstream**, and the
consumer's own independent check is the `good/`+`bad/` pair, which `init` vendors
along with every other kit file and which discloses a gate's *shape* without its
predicate. It also means the gate's consumption path assumes nothing about where
the binary came from: the invocation is through `GATE_SDK_NATIVE_BIN`, the knob
every other binary reader uses, and an installed artifact answers `--reads`
identically to one `cargo` produced.

**The set was re-derived at §The second budget batch's cut and gains no row.**
That batch's README-roster member already carried the row above and the port does
not move it; its commit-message sibling couples `gate-sdk/lib/gate.sh` alone,
which covers **no** registry member's declaration path, so it earns none — a
reverse trigger on the roster's config home, the shape the `check-docs-cmd` row
records. Written down because a port *changes* declaration paths, so the verdict
is taken by running the derivation at each cut rather than inherited, and it is
recorded whether or not it moves a member.

Gates whose corpus is kit directories, smoke scripts or hooks
(`check-kit-registration`, `check-smoke-entry-guard`, `check-hook-exec-bit`,
`check-test-hermetic`, `check-assertion-strength`) are not substrate-sensitive
by the derivation above and owe no disposition row. Four of them have since
ported (§The kit-roots `gate_kit_roots` cohort) and the claim is unchanged by
that: this table is about whose *assertion* a port could end, never about which
members are on which substrate.

**Reference-only implementations.** The section carries one further disposition,
in the other direction: an implementation the binary carries that **no descriptor
dispatches to**. Assertion B's default reading of that is "a gate nothing
declares" — dead code, or the residue of a half-finished port — and it is red.
The exception is an implementation deliberately kept ahead of any live port, so
that the substrate stays exercised by real tests rather than sitting unproven
until the next port needs it. That is a disposition, not a licence: naming it
here is what distinguishes it from the stranded implementation the assertion
exists to catch, and a subcommand not named here still reds.

**The reference-only roster is empty, and an empty roster is not a lapsed rule.**
It is written out as a sentence rather than left as a missing table, because a
table nobody can see is indistinguishable from a rule nobody kept. Each
first-cohort member held a `reference-only` disposition while its descriptor was
held; the descriptors are live, every implementation is dispatched to, and a
disposition whose subject is dispatched to would be false. What the emptiness
records is that **no such subcommand exists today** — never that the allowance
lapsed. The rule stands unchanged and un-weakened: assertion B still reds a
subcommand no descriptor dispatches to, and the next implementation carried ahead
of its port is named here, in this section, or reds. Reading the emptiness as
permission is the vacuity this section exists to refuse.

## Porting a gate to the binary substrate

Slice 1 ported one gate end to end and then **reverted the port**, keeping
everything else it built. This section is the record of what that cost bought,
written here rather than left in a commit message because a later reader
deciding whether to port will read the spec and would otherwise re-attempt it
into the same wall.

**One of the grounds a port is argued on has changed since, and it changed by
ruling rather than by drift.** The slice was built under the constraint that
opacity is not claimed, so the case for a port rested on the seam and the
conservation contract alone. §Consumer payload now rules withholding a gate's
predicate a goal, which makes opacity a ground a port may argue on — bounded
exactly as that section bounds it, raised cost of analysis and never
confidentiality. A later session weighing a port weighs that ruling, not the
constraint set this record was written under.

### The decisions this substrate already closed

Two questions behind the port are settled. Both are recorded here because the
only surface that ever held them was a queue entry, a compression dropped each,
and each was then restated from memory rather than read — so this subsection is
the two of them landing where the component that depends on them can be read
alone. Reading them does not reopen them. The project-wide register of closed
rulings is TRAJECTORY.md, which points here for these two rather than restating
them; the authoring rule that would have prevented both losses is stated where
it belongs, queue-kit/SPEC.md §check-queue-entry-budget, rather than here.

**The substrate language is Rust — ruled 2026-08-02, final.** The alternative
weighed and refused is **Go**, on three grounds: larger binaries, poorer memory
management under a primitive garbage collector, and a weaker compiler. The first
of those is not a matter of taste here, because footprint is a first-class cost
under the objectives the trajectory pivot records
(TRAJECTORY.md §The objectives) and the payload ships one prebuilt binary per
declared target
(§Consumer payload), so binary size is paid per target on every adopter's
machine. The comparison had been made in an earlier session and lost with the
compression that dropped it; it is written down rather than left to the next
session's judgement.

**A bash portability floor was costed and rejected — closed, not deferred.**
What was costed: shimming or dropping the GNU-only constructs the battery
depends on — the gate library's nameref and `realpath --relative-to`, gawk's
three-argument `match()`, `sort -V`, `date -d`, `stat -c`. The blast radius was
about 25 files, mostly single call sites, with two shared libraries carrying
most of the leverage, for an estimated two to four small units. It is rejected
because it cannot deliver the reach it would be bought for: stock macOS ships
bash 3.2, which no shim reaches, and `mapfile` / `declare -A` appeared
in 57 of the 96 checks counted that day — so the floor buys BSD-userland
tolerance and never a stock-macOS install.

**That rejection stands on the trajectory pivot, and deliberately not on this
section's own case for a port.** `native-gate-binary-port` ruled platform reach
out as a ground for porting, so an argument from reach would contradict the
entry it sits under. The pivot reinstates reach: it names every major operating
system and makes the non-technical adopter a design constraint (TRAJECTORY.md
§The objectives, cited above). Read against those, a floor reaching BSD
userlands but not a stock macOS install fails an objective rather than a
preference — and the pivot's own direction, shrinking the interpreter surface to
the unavoidable, is the one this costing already pointed, which makes the
costing a step on the trajectory rather than a casualty of it. The narrower
installer-and-probe trade this does **not** close stays
`install-path-gnu-userland-undeclared`'s.

The figures above are a **dated measurement, not a live claim** about this tree:
they record what the floor would have cost when it was rejected. A later session
must not refresh them against the current battery — a recount is a step toward
re-deciding, and only the operator reopens a closed ruling.

### The non-gate arm

The binary is a multi-call binary whose *gate* subcommands are dispatched by
name out of `gates::REGISTRY`. It also carries arms that are **not** gates —
`--list`, `--reads`, `--knobs`, `--source-stamp`, `--queue-parity`,
`--declaration-parity`, `--evidence-lib-parity` and `--install`, plus the
`--emit-` family the bridged-arm table keys (`--emit-drift-report` is its
2026-08-29 member) — and the class
they form is named here because a
session arriving with a new non-gate thing to port has no other way to learn
that one exists or what it costs. Each arm's own `spec:` comment explains that
arm's placement to whoever is already reading it; none of them can reach the
session that has not started.

A **non-gate arm** is specified by three properties:

- **It is a top-level `--`-prefixed flag, resolved in `main` before the
  registry lookup, and it is absent from `--list`.** The flag's spelling and the
  front-end's grammar are **one decision, not two**: `bin/run-gates.sh` composes
  `--emit-<name>` from its `--emit <name>` operand, so a member spelled anything
  else is reachable by no shipped front-end and is callable only against the
  binary directly. Read the `--emit-` prefix as load-bearing rather than as the
  family's house style. The placement is
  load-bearing rather than stylistic: §check-gate-substrate-parity assertion B
  equates the `.gate` descriptor set with exactly the roster `--list` prints, so
  an arm inside that roster would read as a subcommand nothing declares and red
  the gate. Staying outside it is what keeps that equality true in both
  directions.
- **It owes no `.gate` descriptor, no `gates.list` registration, and no
  `good/`+`bad/` fixture pair.** Those three are the *gate* contract and they
  attach to a thing that returns a verdict a battery reads. An arm that returns
  a document has no pass and no fail to fixture.
- **It owes a named caller instead.** A gate's reader is the battery; a non-gate
  arm has to name the caller that reads its output and the transition where it
  is read, or it is dead weight. Every member above satisfies this —
  `--source-stamp` is read by §check-gate-binary-fresh, `--queue-parity` and
  `--declaration-parity` by their parity harnesses — and stating it is what
  stops the class becoming a place to park unreachable code.

  **A *caller* is the requirement; a stored projection is one shape of it, not
  the shape.** The emit arms that opened this class were all generators of a
  committed page, so their callers were a regen command and a freshness
  comparator, and the rule read as if a member owed a projection on disk. It
  does not. `--emit-close-surfaces`
  (lifecycle-kit/SPEC.md §The close-surfaces emit arm)
  is the worked counter-instance: nothing stores its roster and
  nothing must — the roster's whole value is that it is recomputed at the moment
  its reader asks, so a surface added yesterday appears today. It is a member in
  good standing on two live callers, a gate calling it in-process and a stage
  step running it through the front-end, which is the whole of what this bullet
  demands.

  **A member may also be a *query* tool rather than a generator**, which is the
  reading `--emit-close-surfaces` opened and `--emit-queue-index`
  (queue-kit/SPEC.md §The queue-index arm) completes: that arm's `extent` mode
  answers two integers about one entry and emits no document at all, so the
  `--emit-` spelling reads as a stretch. It is precedented and taken rather than
  renaming the family, which would be a gate-sdk unit of its own. `extent` is
  also the class's worked instance of a mode whose only caller is **a session** —
  nothing in tree invokes it but its own fixture, and the caller is load-bearing because
  §The queue-index arm's refusal to ship a queue-mutating tool rests on it. A
  session reaching a mode through the front-end counts exactly as a stage step
  does.

**The class gained no member from the consumer smoke, and the near miss is
recorded because the next reader will size that harness the same way.**
`bin/run-consumer-smoke.sh` is a `bin/` tool with a named caller and a document
to emit, so it reads as a candidate on every property above. It is not one: its
registration accounting probes each unregistered gate through `gate_command`
(§lib/gate.sh), which resolves that gate's knobs by sourcing the owning kit's
`lib/*.sh`, and a crate-side arm doing the same would be the second producer
criterion 6 refuses. The harness declares `# no-port:` on that ground instead
(§Consumer smoke, *The port disposition*).

**A gate argument that selects where configuration comes from cannot survive a
port, and the reason is an ordering rather than a limitation.** `gate_command`
resolves every knob a member declares *before* it execs the binary, so an argument
whose job is to point the kit library at a different config file arrives a process
too late: the knobs are already resolved from the tree's own config. Such an
argument is not ported and not reimplemented — it is **deleted**, because a
documented flag that silently changes nothing is worse than no flag, and its
callers are re-pointed at the config path the knob resolution already reads. The
worked instance is `check-docs-cname-parity`'s third positional
(site-kit/SPEC.md §check-docs-cname-parity), whose fixture pair moved to a
gates-dir `site-config.sh` at no cost; §The third budget batch's `--fixture`
deletion is the same shape reached from a different direction. A porting session
should check for this shape **before** sizing a member, since it is an interface
removal on a governed surface rather than an implementation detail.

**The distinguishing test, stated as a test because it is applied far more often
than the deletion happens.** An argument is unportable when it redirects
something `gate_command` has **already resolved** from the tree's own config
before the exec — it arrives a process too late and would silently change
nothing. An argument the **rule itself** consumes, arriving as argv into the
subcommand, ports unchanged: the subcommand reads its own argv and may override
a bridged knob value with it perfectly well. It is not a blanket deletion, and
the reading that makes it one is the error to avoid — `check-root-tiering` kept
**both** its positionals through its port (§The third budget batch), so an
argument is not unportable merely by being an argument.

**The verdict is taken per member against that test, never off a count.** The
fifth batch's cut carried a command-line argument on six of its seven members
and read that as six deletions; applied per member the ruling bound **twice**. A
**gates-dir positional** is consumed by the rule — it names the registry and the
first resolve dir — and ports unchanged, as `check-kit-enum` and
`check-root-tiering` had already established; so does `check-gate-binary-fresh`'s
tree-stamp file, which is the rule's own input, and `check-gate-tamper`'s
`--fixture <dir>`, which selects the rule's input *corpus* and which no knob
resolves. A **queue-file or state-file positional** falling back to a knob
(`check-amendment-queue`, `check-evidence-manifest`) ports unchanged for the same
reason, the shape `check-todo-task-liveness` already carries in tree. The two
that deleted are `check-identity`'s two arms (§check-identity) and
`check-gate-fixture-coverage`'s positionals, whose ground is narrower and stated
at that member's own section. A relayed count read as a target is how a batch
talks itself into deletions it never verified.

**§The sixth budget batch applied the same test and it bound zero times, over six
arguments on five members.** Two shapes cover all six and both port unchanged. A
**scan-root positional** — a root no knob resolves, which the rule composes with
the bridged globs it is handed — is the shape three members carry
(`check-prose-tells`, `check-spec-embedded-source`, `check-template-copy-parity`),
and it joins the worked instances above as an argument that ports. An
**input-corpus positional** — one that selects what the rule analyses rather than
redirecting resolved config — is the other two
(`check-reads-couples`' gate-source paths, `check-gate-exemption-tasks`' scan
dirs), the shape `check-gate-tamper`'s `--fixture` already establishes. That
member's queue-file positional falling back to a knob is the shape named by name
above. Two of the six are documented on a governed surface and both sentences stay
true, which is the finding rather than an aside: the doc obligation in the
paragraph below binds only on a deletion, so a cut with no deletion owes no doc
edit — and the batch that read six arguments as six deletions is the reason that
is worth saying twice.

**Where a deleted argument was documented, the doc is part of the port.** A port
that deletes an argument and leaves the sentence standing ships a documented flag
that does nothing, which is the state this section calls worse than no flag.

**An arm receives no configuration, and a member needing some is reached
through a caller.** The config bridge is built by `gate_command` (§lib/gate.sh)
for a `.gate`-declared member alone, and `kit_roots` is transported rather than
re-derived by standing crate invariant, so a bare invocation of an arm resolves
only what the arm can compute for itself. This does not make the class unusable
for a member with inputs: the arm stays the only entry point into the crate, and
a front-end that already sources the shell library — the battery runner among
them — supplies the bridged environment in front of it. What the class forbids
is a *second* entry point into the emission path, not a caller.
**So the family choice is forced for any tool that needs configuration at all.**
Only a **bridged-arm table** member is bridged: the table's knob list is what
`--knobs` prints and what a front-end resolves, while a
hardcoded top-level flag is reached by neither and receives nothing. A configured
tool ported as a top-level flag therefore resolves platform defaults and silently
ignores every consumer override — which is not a calibration between two workable
shapes but the difference between working and appearing to.

**A default the deleted shell driver held inline moves into the owning kit's
library in the same cut that deletes the driver, never after.** The bridge
resolves a declared knob by sourcing exactly one kit's library (§lib/gate.sh),
so a default left beside the compiled reader is sourced by nothing and resolves
empty — which the reader takes as an unset knob rather than as an error, so the
failure is silent. The library is the knob's only home from the moment the
shell home goes; drift-kit's report knobs are the worked instance.

**`--install` is the class's first *deliberately unbridged* member, and that is
the property the class had not carried before.** Every earlier hardcoded flag is
hardcoded because it needs no configuration; this one is hardcoded because its
caller cannot supply any. The forced-family test above resolves on what the
member reads, and `--install` reads nothing: it is called by the installer's
bootstrap, which TRAJECTORY.md §The objectives' objective 6 forbids assuming to
be a POSIX shell at all, so **every value it needs arrives as argv** and it
resolves no knob and no kit config. A bridged install arm would be resolved by
`gate_command` — a bash front-end sourcing each owning kit's `lib/*.sh` — and
would therefore be unreachable from the PowerShell half of that boundary. Its
named caller is `installer/lib/init.sh` today and its PowerShell twin after,
and its grammar, channels and exit statuses are
installer/README.md §The install boundary's. It owes no descriptor,
registration or fixture pair, like every other member; what it is asserted by is
the consumer smoke's install path (installer/README.md §The consumer smoke).
**The zero-config reading is a property, not an exemption** — a later op that
grows a configured input has to arrive as a new argv key rather than as a knob,
or it re-breaks the half of the boundary the member exists to serve.

**`--emit-port-blockers` is a landed member of that table, and the route it took
was recorded here before it was taken.** The port oracle reads the gates dir, the
kit roots, the prune set `GATE_PRUNE_DIRS` / `GATE_SDK_PRUNE_EXTRA_DIRS` resolves,
`GATE_SDK_PROGRAM_FLOOR` and the fixture-dirs root criterion 2's column reads
through, so the forced-family test above settled its family with nothing left to
calibrate — a top-level flag would have resolved platform defaults and ignored
every one of those overrides. The arm's own argv is the three arms' spellings
unchanged, plus a `--gates-dir` the rule itself consumes, so the distinguishing
test bound zero times and nothing was deleted (§port-blockers).

**The flag spelling was refused until the porting unit minted it, and this is
the record of that mint.** This section rules that a member owes a named caller;
the closed-roster rule that a key or a field is minted *with* its reader or not
at all applies to a flag spelling identically, and a spelling written into a SPEC
ahead of its implementation is a reservation. The spelling is
`--emit-port-blockers`, and it was not a third stretch of the word *emit*: all
three of its modes render a roster, and the prefix carries the reachability,
because `bin/run-gates.sh --emit <name>` composes the flag as `--emit-<name>` and
a differently-spelled arm would have been reachable by no shipped caller at all.
**That unit's caller was known ahead of it and recorded here as a constraint:** a
consumer's measured-claim emitter reads the `--tree` trailer (§port-blockers) and
had to move to the arm in the very commit that deleted the shell tool, or the
consumer's claim oracle exits 2 — a failure landing at an adopter's battery
rather than in the crate, which is why the constraint was recorded where the
route was rather than left to be discovered.

**The declared knob roster carries a union sentinel, and that is the read the
recorded route did not account for.** The route's list of what the tool reads named
**four reads and was one short**: beyond the structural knobs — those four, plus
the fixture-dirs root criterion 2's column reads through — the tool resolves an
*arbitrary* knob's default, one discovered at scan time from a command-position
expansion or named by a `?` row of a compiled member's requirement declaration.
A bridged arm receives only its declared knobs and the bridge refuses an
undeclared read, so a fixed roster cannot answer. All three fallbacks are refused: reporting `?` for every expansion loses
rows the tool resolves today, parsing kit config inside the crate is a second
definition of the one resolver, and re-entering the shell library from the
compiled arm is the thing the port exists to end. The mechanism is the union
sentinel `check-reads-couples` already declares when its knob names are not known
until run time, minted here as a second member of that class: it expands to the
union of every knob every member of the **tree's** registry declares, scoped by
the arm's own `--gates-dir` argv for the correctness requirement `--run`'s union
is scoped for. The union is provably sufficient — a command-position expansion
naming a knob its own member does not declare already fails the bridge's
undeclared-knob refusal at that member's own dispatch — so a knob outside it is
reported `?` with the same *default unresolvable* evidence, the fail-safe
direction unchanged.

**The dispatch union keys on the arm's *variant* rather than on the member, and
the sentinel is what let a second member express a union without moving it.**
`knobs` answers `Arm::Run` with the dispatch union and `Arm::Emit` with the
member's own roster; that is exact today because one member carries the
dispatching variant, and the port did not have to widen it, because a sentinel in
a member's declared roster expresses the same union per member. Recorded because
the two mechanisms look interchangeable and are not: the dispatcher hands each
*child* its own slice, where the sentinel resolves a union the arm itself reads.

**A derived knob set is not always the union sentinel, and the two are near
enough to be confused.** drift-kit's collator (drift-kit/SPEC.md §The report
skeleton) arrived at this class with the same shape of problem — its contract is
built on `compgen -v DRIFT_KIT_` precisely so that no fixed export list can drift
out of parity, so a transcribed roster would replace a derivation and land on the
derivation-first rule. The sentinel is **not** its answer: the sentinel is scoped
to the *gate* registry's members, and that arm's registry is a KPI list whose
members declare no knobs at all. Its answer is the **prefix family** `DRIFT_KIT_*`,
which the bridge resolves by running that same `compgen` inside the owning kit's
already-sourced subshell (§lib/gate.sh) — the shell contract moved rather than
re-implemented, on `--emit-enforcement-map`'s `EVIDENCE_KIT_RUN_*` precedent.
Recorded because both mechanisms answer "the roster is not knowable here" and
only one answers "the roster is the namespace".

**Most of what that port needed was already in the crate**, which was the other
half of why the route was recordable ahead of it. `registry.rs` owns the registry
walk both registry arms run on; the prune resolution and the `--tree` corpus
rule's own compiled face were shared crate mechanism too, the latter no stub
built ahead of demand — §check-gate-exemption-tasks' tree arm had called it since
that arm landed. What the port owed beyond them was the header-block read, which
lived inside the gate module that needed it first and was promoted to the
universal layer beside that corpus rule rather than duplicated; the fixture-dir
resolution, extracted from §check-gate-fixture-coverage and shared with its roots
as a parameter; the three arms' trailers; and the command-position tokenizer,
which existed nowhere and was the port's cost centre.

**That is why the table is named for *bridged*, not for *emit*.** What its members
share is not that each renders a document — `--emit-queue-index extent` answers
two integers and emits none — but that each publishes a knob roster a front-end
resolves. The table is therefore keyed by the **arm's own flag spelling**, and
`--emit-` is demoted from a family name to a per-arm spelling: nothing about the
existing arms changes, and a non-emitting member can join without a third stretch
of the word *emit*. The rename is the one this section declined to take for
`extent` on the ground that it was "a gate-sdk unit of its own"; it is taken here
by the unit that needed it, as a generalization of the property already stated
rather than as a new class.

**`--run` is the class's first bridged member that returns a verdict rather than
a document** (§run-gates). It satisfies the three properties: it is
`--`-prefixed, resolved before the registry lookup and absent from `--list`; it
owes no descriptor, registration or fixture pair, because it returns no verdict a
*battery* reads; and it names its caller, `bin/run-gates.sh`. It is a table member
rather than a hardcoded flag for exactly the reason above — it reads the gates
dir, the kit roots and the scratch dir, so a flag would be the second shape.

**Its declared knob roster is derived, never maintained.** `--knobs --run` answers
the **union** of the runner's own knobs with those of every member the *tree*
registers, computed from `gates.list` and the crate's registry. A maintained
union would rot against a churning roster the moment a member's knob set changed,
which derivation-first forbids; the union is exactly the data `gates::knobs` and
the registry already hold. The dispatcher then hands each child only its own
member's slice of that union, which is what keeps the declared-knob discipline
executed rather than assumed (§run-gates).

**The scope is the tree's registry rather than the crate's, and that is a
correctness requirement rather than an economy.** The binary carries every ported
member whatever profile a consumer installed, so a union taken over
`gates::REGISTRY` asks the bridge to resolve a knob whose owning kit is not
vendored — the does-not-define refusal, fail-closed, on the adopter's very first
battery. The starter profile is the worked instance: one kit vendored, a
`gates.list` naming only its members, and a crate-wide union demanding
`CANON_KIT_ACTIVE_SECTIONS` from a tree with no canon-kit. The arm's own argv
therefore reaches `--knobs`, because the registry it will walk is the only thing
that scopes the answer correctly, and `gate_knob_env` forwards it for that
reason. **A sibling arm's knobs are not in the union either**, for the same
property read forward: the runner never invokes a sibling arm, so declaring its
knobs would violate the very rule `--knobs` exists to hold. A gate that reaches
an emitter in-process declares that emitter's knobs on its own registry entry,
which is the transitive-coupling rule below already covering the case.

**A gate that reaches an arm in-process acquires a source coupling its
descriptor must carry, and the trigger set does not follow the port on its
own.** In the shell form a comparator spawned its emitter, and the spawn was
invisible to the `# graph:` manifest — a coupling nothing could have declared,
because the manifest names tracked paths and a subprocess is not one. In the
compiled form the same relationship is a function call across crate modules, and
it *is* nameable: the callee's module is a tracked source file whose edit changes
the caller's verdict. So the porting session owes the descriptor every module its
gate reaches, transitively, including a module shared by both sides of a compare.

**"Transitively" stops at the universal layers, and the tree is what says so.**
Read at face value the rule reaches `walk.rs`, `proc.rs` and `registry.rs` — the
config-bridge, spawn and registry layers every gate module reaches — and no
descriptor in the tree names any of them,
correctly: coupling a universal layer into every descriptor spells one
fact once per ported member and re-runs the whole battery from the generated hook
on any edit to it, which is de-literalization inverted. What the rule reaches is
the modules whose edit can change **this** member's verdict and nothing else's —
its own module, and the shared rule-carrying modules beside it (`fresh.rs`,
`declaration.rs`, an emitter it calls in-process). The universal layers are held
by §check-crate-arms and by the binary's own source stamp instead, which is the
same coverage through a mechanism that does not scale by descriptor count. Stated
here rather than left to each porting session, because the sentence above invites
the literal reading and the tree silently contradicts it.

**`registry.rs` is the third such layer and it arrived by collapse rather than by
design.** It owns the `gates.list` member grammar, `gate_resolve`'s declaration
path (dirs consumer-first, `.sh` beating `.gate` within a dir), the `# graph:`
field read and the `couples=`/`trigger=` kit expansion — one implementation for
the readers that each carried a private copy of it: `check-gate-binary-fresh`,
`check-kit-enum`, `check-gate-fixture-coverage`, `check-core-files`, the graph
projection and the enforcement map. A registry walk is what the `--run` arm
needed and what those five had already duplicated, so the module is the
de-literalization that port made unavoidable rather than a layer invented for it.
Omitting them leaves the gate registered and green while the projection it holds
goes stale at commit time, because the generated hook's `staged_matches` trigger
is derived from `couples=` — the gate simply never runs on the edit that broke
it, and only a full battery finds it. Stated here rather than in a member's own
row because it is a property of the port, not of any one gate. The freshness
family discharged it in full: every member's emitter now lives in the crate
beside its comparator, and each descriptor carries the modules the call reaches
— two of them acquired from behind, `check-docs-mirror-fresh` having *triggered*
on its generator without coupling it and `check-trajectory-fresh` having named
its extractor in neither field.

### The port-candidate criteria

**These seven are an engineering roster and an ordering signal, never an
eligibility screen.** The operator ruling of 2026-08-09 (TRAJECTORY.md §PRIORITY
DIRECTIVE — the port track's sequence) ports the whole corpus, so none of them
excludes a gate from the port. Each does two jobs instead: it **names an
engineering problem the port owes** for the gates that fail it, and it **orders
the work** — a gate clearing all seven is cheap to port today, a gate failing one
carries exactly the cost the criterion describes. The first four were stated at
design time; the last three were paid for, and each is named with what it cost.

**All seven bear on *gates*, and the directive's completion predicate bears on the
*tree* — a distinction worth stating because the criteria are the natural thing to
reach for and most of them have nothing to say.** A criterion asks after a
registration, a fixture pair, a tier, a `couples=` — properties a gate has because
it is a gate. A plain tracked script has none of them, so applying a criterion to
one yields not a hard question but an unanswerable one, which is why §port-blockers'
`--tree` arm emits no criterion column over that corpus. The predicate asks a
different and much smaller question of each file: is there a stated disposition. The
two rosters are read at different transitions by different sessions, and the arm
that answers each is the one whose corpus matches its question.

1. **Registered** in `gates.list` — an unregistered gate proves no dispatch.
2. **Carries a fixture pair** — parity between substrates is proved by running
   both against the same cases, never asserted.

   **A `# no-fixture:` member satisfies this by a constructed scenario, not by
   an exemption.** Where the state under test has no static representation — a
   HEAD-vs-worktree diff, whose committed fixture has HEAD == worktree — the
   pair is unavailable but the criterion's actual demand is not: *the same
   cases, both substrates, while both implementations exist*. The port
   therefore stands the state up in a throwaway tree, runs both implementations
   over it, and compares bytes and exit codes. That is a **stronger** oracle
   than the pair it replaces, not a weaker one, because the scenario reaches
   branches a committed case cannot — the loss case itself, and the
   no-repository branch most parity harnesses skip. It is bought once, at port
   time, with delta (9)'s standing limit: it proves the two agree then, and
   nothing machine-held keeps them agreeing after, which is why the shell
   original is deleted for a ported member rather than left running beside it.

   **The second worked instance is usefully different, and the contrast is the
   part to read.** The first is a member with *no* static representation at all.
   `check-memory-off` **had** a pair — what it lacked was a pair that reached the
   ported derivation, because its `--fixture <dir>` arm redirected every path a
   knob already redirected and so drove a second code path the live arm never
   took. The discharge there is therefore in two halves: the arm is **deleted**
   and the pair re-pointed onto the knobs, which fixes the half a fixture can
   fix, and the scenario covers only the residue — the default derivation, whose
   corpus is the harness memory dir under `HOME`. Stated so a later member does
   not cite this as licence to replace a feasible pair with a scenario: where a
   pair *can* reach the derivation, making it do so is the discharge and the
   scenario is what is left over. Such a scenario may also carry an arm that
   cannot be compared at all — one **retired** by the port rather than proved
   equal — and recording it as retired is what keeps the run's verdict from
   reading as a clean sweep it did not have.
3. **`tier=precommit`** — it lands in the generated hook, so a green
   `check-graph` after the port is end-to-end proof the manifest survived the
   substrate change.

   **The literal value is a proxy for that reason, not a bar, and a
   `tier=commit-msg` member satisfies it in full.** Such a member lands in the
   generated **`commit-msg`** hook, which `check-graph` holds against
   `--emit-commit-msg` on identical terms, so the end-to-end proof this criterion
   exists for is available on the same terms. What the criterion actually names is
   *lands in a generated hook `check-graph` holds*; the two came apart at the first
   `commit-msg` port and the adjudication is recorded at §The second budget batch,
   which also records that the emitter and the config bridge needed no widening to
   reach that tier. `tier=align-only` is untouched by this: it emits into no hook,
   so the reason has nothing to attach to and the criterion still names a real cost.
4. **Its assertion target is not gate source** — porting a gate that audits gate
   sources makes the parity proof self-referential: the corpus the
   cross-substrate comparison runs over is changed by the very port being
   compared.

   **The discharge is the fixture corpus, and it is general rather than one
   cohort's trick.** A member failing this criterion still ports; what it owes is
   an oracle the port cannot invalidate, and the pair is the only corpus with that
   property — `gate-tests` is pruned from every live-tree walk, so no port can add
   or remove a file inside it (§Fixture-pair discipline, which owns the widening
   rule and its `expect.txt` discipline). The condition is that the pair carry
   **every arm of the derivation being ported**: a gate-source auditor whose cases
   are all `.sh` proves the `.sh` branch and leaves the descriptor and
   implementation branches resting on a live tree that is green because it is
   clean. Widen first, then port — a port answering only the criterion's sentence
   ships the hole the sentence was pointing at.

   **A criterion-4 bind therefore prices a port; it never holds one.** The
   sentence above already says a failing member still ports and already names the
   discharge, and *widen first, then port* is an **instruction**, not a blocker:
   a member whose only unmet criterion is 4 is **takeable**, carrying the widening
   in its price. Saying so is not a new ruling but the consequence the criterion
   never stated, and it is stated because the omission produced a real declaration
   — a correctly identified criterion-4 bind spelled as a hold, on a field whose
   meaning is *not takeable now*. The evidence is five members rather than the
   argument: `check-docs-cname-parity`, `check-gate-exemption-tasks`,
   `check-knob-default-coupling`, `check-spec-embedded-source` and
   `check-gate-tamper` each bind this criterion and each **ported**, discharging
   it by widening the fixture pair. So a member binding it declares no
   `# port-until:` on that ground (§The `# graph:` manifest owns the field), and
   the shared-snapshot **ordering** constraint the paragraphs below rule is
   narrowed by none of this: a cohort still sequences a criterion-4 member
   deliberately, it just does not declare it held.

   **The second worked instance is a README-roster auditor over `checks/`
   basenames**, whose pair carried only `.sh` kits, leaving the `.gate` arm the
   shell already implemented correctly exercised by no case at all. The widening
   that discharged it is the general shape rather than that member's trick: a kit
   whose `checks/` holds a descriptor, **and** a mixed `.sh`+`.gate` kit, which is
   what proves the union of two globs rather than either alone (§The second budget
   batch).

   **The live-tree arm is demoted from proof to smoke for such a member, and the
   demotion is recorded here so the next gate-source cohort inherits it.** Earlier
   cohorts proved parity on the fixture pairs, the live tree and the edge roots and
   treated the three as one oracle. For a member whose assertion target is gate
   source the live-tree arm *cannot* be a proof: assertion A forbids a descriptor
   and a script coexisting in one resolve dir, so the comparison necessarily runs
   on the pre-descriptor tree — a corpus the port then changes, with no second
   implementation left to notice a disagreement. The arm is retained, because it is
   cheap and it finds real disagreements; its verdict is recorded as **no
   disagreement found on the pre-descriptor tree**, never as *parity proved*.
   **The demotion is escapable, and `shell-gate-tail-port` is where it was
   escaped**: assertion A binds a *resolving* name, so restoring the pre-port rule
   under a name no registry member resolves to lets both implementations run over
   the **post**-descriptor corpus — the corpus the port actually produces — and a
   member taking that route records *parity proved* on the live tree rather than
   the demoted verdict (§check-install-disposition). Where the port also moves a
   *sibling* member's corpus the demotion still stands, because there the two
   implementations read two different trees. The
   edge-root arm keeps its own separate value and the comment cohort is the worked
   case: it disagreed, and the compiled side was the correct one — a `..` scan root
   made the shell's kit-root prune compare an unnormalised file path against a
   normalised root and prune nothing at all. A disagreement is a finding to
   adjudicate against the rule, never a defect in whichever side moved.

   **This is not §Meta-gate conservation's *substrate-sensitive* set, and reading
   the two as one term is the defect this wording exists to close.** That set is
   derived from a member's expanded `couples=` and is deliberately **trigger**-shaped:
   assertion C's job is anti-vacuity — a member whose *re-run trigger* reaches a
   gate declaration must carry a disposition, so a port cannot silently end an
   assertion — and over-selection there costs one table row, which is why it is
   wide on purpose (§check-gate-substrate-parity). Criterion 4 asks a different
   question and binds where **a registry member's declaration path lies inside
   the corpus the gate scans as content**. A member the derivation selects fails
   criterion 4 only where that is *also* true of it, and there are two ways it
   commonly is not: a **reverse-trigger** couple, named only so that a script
   edit re-runs the gate, is never read as content at all; and a **content couple
   wider than the walk**, whose glob covers declaration paths the gate's own
   population predicate never reaches, is content-shaped and still not over gate
   source. A conservation row and a criterion-4 bind are therefore independent
   facts about a member, which is what they always were.

   **The worked instance was machine-derived for as long as a `.sh` declared a
   gate anywhere, which is why this predicate is stated here rather than reasoned
   out per port.** Assertion C's derivation reported
   `check-template-registry-parity` substrate-sensitive: a `kit:` token expands
   **once per kit root** (`gate_expand_couples_var`), so `kit:*/*.sh` is
   `<root>/*/*.sh` for every root, and it covered every `.sh` still declaring a
   registry member under **any** kit's `checks/` — a shrinking set named by its
   shape rather than by a member or a root, because a port empties it one file at
   a time and empties whole roots on the way. `gate-sdk/checks/` is the root that
   emptied first, at `shell-gate-tail-port`'s `check-crate-arms` port, and the
   derivation kept selecting this member through the roots that had not. **That
   set is now empty**: `shell-gate-tail-port`'s last registered member,
   `check-docs-render-fidelity`, emptied `site-kit/checks/` and with it the last
   `.sh` gate declaration in the tree, so a `*.sh` glob covers no declaration path
   and the derivation selects this member nowhere. Its row at §Meta-gate
   conservation stands as a **recorded** disposition rather than a re-derived one,
   which assertion C admits because it asserts derived ⊆ dispositioned and never
   the converse. The verdict below is the one taken while the derivation still
   selected it, and it is kept because the *predicate* it teaches is what this
   criterion needs and does not expire with its instance. It is the second kind of
   over-selection above — the gate does read `*.sh` names as content, but only
   under a `<kit>/<name>/` directory that a sibling `<kit>/templates/<name>.list`
   registers, and no kit ships a template registering `checks/`. Against this
   tree that walk reaches one live registry, drift-kit's `kpis.list` — held in
   population by native dispatch rather than by a sibling directory since the
   2026-08-29 cut — with `gate-sdk/templates/msg-patterns.list` skipped for want of
   either — neither holding a gate declaration, which is what its conservation row
   already records.
   Under the borrowed term the member reads as a criterion-4 failure and under
   criterion 4's own predicate it does not. The corpus is derived from the tree,
   so the verdict is taken by running the derivation at cohort-cut time, never
   read off this paragraph: a consumer whose kit ships `templates/checks.list`
   beside its `checks/` would put declaration paths inside that walk, and there
   criterion 4 would bind.

   **The instance in the opposite direction — the couple clears and the walk
   binds — is `check-docs-cname-parity`, and it matters more than the two above
   because it produces a *missed* hold rather than an over-selected row.** Its
   `couples=` is one literal file, so the derived substrate-sensitive set does not
   select it and assertion C structurally cannot report it; its walk nonetheless
   reaches every tracked file, because its scan root defaults to the whole tracked
   tree and the rule greps each file's bytes. Every kit's `checks/*.sh` and
   `*.gate` is therefore inside the corpus it scans as content, which is this
   criterion's predicate exactly. The lesson is the one the paragraph above
   already states from the other side: a conservation row and a criterion-4 bind
   are independent facts, so the verdict is taken by **reading the walk**, never
   by reading the trigger field (§The fourth budget batch; site-kit/SPEC.md
   §check-docs-cname-parity owns what it cost that member).

   **The second machine-derived instance is the other over-selection kind**, so
   both now have one. Run at the ERE cohort's cut, assertion C reports
   `check-install-claim` and `check-payload-claim` substrate-sensitive through a
   `couples=` glob `scripts/*.sh` that covers a consumer's gate wrapper. That
   couple is a **reverse trigger**: it exists so an edit to the consumer's
   transport and disclosure emitters re-runs the gate consuming them, and neither
   gate reads a declaration path as content — what both scan is the governed
   markdown set. Criterion 4 binds on neither, and both carry the conservation row
   the reverse-trigger case earns.

   **A verdict can flip on which *consumer configuration* is read, which makes
   this a property of a gate against a config rather than of a gate.**
   `check-gate-tamper` is the worked instance, and
   delegation-kit/SPEC.md owns what it cost that member.
   Under the kit-shipped `DELEGATION_KIT_GATE_FILES`
   default its corpus is the consumer's own gates directory, which holds no kit
   declaration, and criterion 4 **clears**. Under a config widening those globs
   to every kit's check dir — this repo's — the gate's own declaration falls
   inside them, and staging it makes the gate read its own bytes, so criterion 4
   **binds**. Reading the kit default is the natural first stop and gives the
   wrong answer for the tree the port actually runs against. It produces the same
   failure as the couple-clears-walk-binds instance above, a missed hold, through
   a third route: not a wrong field and not a wrong walk, but the right walk over
   somebody else's configuration.

   **A member with *no clearing configuration* is the register's last shape, and
   §The sixth budget batch produced the first.** `check-gate-exemption-tasks`
   globs both declaration spellings into an in-scope set and an out-of-scope set,
   and reads both: in an authoring tree its own declaration is in the scanned set,
   and in a vendored consumer it is in the out-of-scope set, which is still read to
   build the skip count. There is no configuration in which the criterion clears,
   so the two paragraphs above about a verdict flipping on a config have a
   counterpart — a member whose verdict flips on nothing. `check-graph` joined it
   at the seventh cut and `check-gate-assertions` at the eighth, so the row has
   three members.

   **`check-gate-assertions` also gives the register a shape it did not have: an
   immunity that is *contingent* rather than structural, and the difference is
   what makes it worth naming.** The criterion has two spellings in this section
   and they gave **opposite** verdicts on that member. Under *a registry member's
   declaration path lies inside the corpus the gate scans as content* it bound
   before its port in every configuration, the gate resolving every enumerated
   contract's heading to a declaration or an implementation module and reading its
   bytes. Under *the gate's **own** declaration path* it cleared — but only
   because its own SPEC section happened to carry no enumerated contract, so
   discovery filtered its own heading out. That is one sentence of prose away from
   ending, and the port is the likeliest author of that sentence, since every
   ported sibling's section opens with an enumerated contract. Contrast
   `check-gate-fixture-coverage` below, whose immunity is a **theorem** its own
   rule enforces. An immunity guarded only by a prohibition **the gate itself
   would have to enforce** is circular, so the eighth cut took the binding verdict
   and ended the contingency deliberately by making the member self-auditing
   (§check-gate-assertions). **The two spellings are not reconciled here**, and
   that is deliberate: settling which one the criterion means is a change to this
   section's own rule with reach across every member, so it is filed
   (`criterion-4-two-spellings-disagree`) rather than taken inside a port. What
   the eighth cut settled is the member, on the conservative verdict this
   criterion already prescribes when a reading is uncertain.

   **That batch also produced a *third* couple-clears-walk-binds instance, and the
   sharpest**: `check-knob-default-coupling`'s `couples=` field is one level deep
   and does not cover its own declaration path, while the recursive kit-root walk
   beneath it — which prunes `templates/` and nothing else — opens that path as
   content on every run. The gap between the field's depth and the walk's is the
   whole mechanism, and it is why the verdict is read off the walk. The same batch
   took one verdict **conservatively without ruling the class it belongs to**: a
   member whose walk content-compares every source file as a *diff reference*
   rather than as its assertion target takes the binding verdict, because that
   costs a fixture widening and cannot be wrong in the harmful direction, while
   clearing wrongly ships the hole this criterion exists to point at. The class
   stays `spec-embedded-source-criterion-4-membership`'s, and a later reader must
   not read the disposition as its answer.

   **`check-tree-terms` joins the register as its *widest* instance.**
   Its `couples=` is one literal pattern file, so the derived substrate-sensitive
   set does not select it; its walk is `git ls-files` over the **whole tracked
   tree**, pruned only by the shared prune dirs and the `msg-patterns` basenames,
   so every registry member's declaration path *and* every implementation module
   sits inside the corpus it greps as content. What makes it the widest is that no
   narrowing is available: `check-docs-cname-parity`'s scan root is a knob a
   consumer can point elsewhere, while this member's corpus is the tracked set
   itself. The bind is therefore read off the walk — the field would clear it —
   and §check-tree-terms owns what it cost that member.

   **And a member can be *self-immune*, clearing the criterion while the
   ordering constraint still binds.** `check-gate-fixture-coverage` reaches a
   declaration's bytes only for a member with **no** resolvable fixture pair —
   and it must carry a pair itself to pass its own rule, so it can never reach
   its own bytes. The criterion's predicate is *the gate's own declaration path
   lies inside the corpus it scans as content*, and for this member the answer is
   no for itself and yes for its siblings. So criterion 4 and the shared-snapshot
   ordering constraint are **independent facts**, the way a conservation row and a
   criterion-4 bind already are: the criterion protects the parity oracle from
   the member's *own* port, the ordering protects every comparison from a
   *sibling's*. The fifth batch is where they came apart, and a session clearing
   one by clearing the other has a verdict it did not earn.
5. **Its vendored form stays runnable.** *Measured, not reasoned.* A `.gate`
   descriptor under a vendoring kit root reaches every consumer; the binary does
   not, because `native/` ships no `checks/` or `smoke/` and that predicate is
   what makes a root directory a kit. That asymmetry has a second worked
   consequence beside the vendored-consumer one: a **harness** that vendors kits
   per ref cannot vendor the binary per ref either, so it must build one itself or
   silently pair one ref's shell with another's artifact (§upgrade-smoke). `gate_command` is fail-closed on an absent
   binary and its exit 2 is a **dispatch-harness** error, so it takes down the
   *calling battery*, not just its own member. A freshly vendored consumer's
   pre-commit battery therefore died on invocation. **The condition that
   satisfies this criterion is ruled**: the payload carries a prebuilt binary
   per declared target, built by the release and never from a working tree; the
   installer resolves the host to a target, verifies the matching artifact
   against a published digest, and copies it — no selection ever builds; and a
   member the host's platform has no artifact for is left out of the consumer's
   `gates.list` and recorded there as an omitted member rather than dispatched
   into an absent binary. The producing half of that model is **built**: the
   publish workflow's roster-derived build matrix emits one binary and one digest
   sidecar per declared target, `scripts/pack-installer.sh` verifies each against
   its sidecar and places them in the payload, and the Release publishes them
   (§Consumer payload). The placing half is **built** too: `init` resolves the host
   to a target, refuses on a digest mismatch rather than warning, and
   omits-and-declares a member whose platform the roster carries no artifact for
   (`native-artifact-install-path`). Both halves of the model therefore ship, **and
   the criterion is met rather than merely ruled**: `v0.22.0` published the binary
   and its digest sidecar as Release assets, so a vendored consumer whose host the
   roster carries an artifact for resolves, verifies and runs the ported member.
   The criterion is satisfied **per target, not globally** — one declared triple has
   a published artifact today, and a host with none is omitted-and-declared rather
   than dispatched into an absent binary, which is the branch that keeps a freshly
   vendored battery alive on an uncovered platform.

   **That branch bounds what the *installer* may relocate, not only whether a
   ported gate stays runnable.** Both no-binary outcomes leave `init` with nothing
   to invoke, so an install step moved behind the binary invoke is a step an
   artifact-less host cannot run at all: the failure the relocation introduces
   there is no install rather than a smaller battery. The rule that follows —
   a step is takeable now iff it already runs only when an artifact was selected,
   and relocating the unconditional remainder is sequenced behind the roster
   covering every supported platform — is
   installer/README.md §The install boundary's, stated there because it governs
   the installer rather than any gate. This criterion is where a porting session
   meets it, which is why it is named here rather than only there.
   The placement branch never waited on that tag for its
   evidence — `consumer-smoke-artifact-arm` gave the consumer smoke a leg that builds
   the binary and packs it, so every invocation drives selection, pre-write digest
   verification and placement against a real artifact; since
   `port-criterion-aggregate-cost-blindness` that artifact rides the **main**
   payload, so every profile the smoke installs takes the placement branch
   (installer/README.md §The consumer smoke).

   **The criterion is priced per member and paid per cohort**, because the
   quantity a per-member statement cannot see is not any member's runnability. A
   cohort's aggregate cost is the **binary-less residual**: what a consumer whose
   payload carries no artifact for its host still catches once every member of the
   cohort is a descriptor. Seven `spec_manifest_files` members ported in one batch
   (`f602642d`) each passed the per-member reading, and the aggregate left such a
   consumer with no gate asserting markdown-link liveness at all.

   **The residual is the omitted roster and its count, and that is what a cohort
   records.** It is **measured, never reasoned**, and the instrument is
   `installer_smoke`'s binary-less leg: it installs a profile from an
   artifact-free payload, derives the set that payload dispatches to a binary from
   the consumer's own vendored tree, and asserts the consumer's `gates.list`
   declares exactly that set, at a **non-zero** count
   (installer/README.md §The consumer smoke). The quantity is therefore
   machine-derived and machine-asserted-complete, and a cohort records **the
   roster it grew**, against the post-cohort registry, with its amendment ruling
   whether that roster is acceptable. **N members each individually runnable is
   not a discharge**, and citing the per-member reading as one is the defect this
   half exists to name.

   **When the instrument rides the same iteration as the cohort, the measurement
   waits for it.** The binary-less leg is what prints the roster, so a cohort
   landing in an iteration that is also repairing that leg measures its price
   **after** the repairing entry's amendment lands. Measuring before it reads a
   residual that is not the cohort's own: an artifact-free profile is then the
   accidental no-artifact case rather than a deliberate measurement, and the two
   are indistinguishable from the outside. A build session batching such an
   iteration inherits the ordering rather than discovering it. A second ordering
   rides with it: the instrument packs a payload stamped with a commit, so it
   refuses a dirty worktree and the measurement runs **after** the cohort's own
   commit — from a clean checkout of it when a concurrent session holds the tree
   dirty. That checkout is reached **by path, and cwd does not select it**: the
   smoke resolves its own tree from its script path and passes it to the packer
   as `--root`, so invoking `<clean-checkout>/installer/consumer-smoke/run-smoke.sh`
   from anywhere measures that checkout. The instruction this paragraph carried
   until `pack-installer-root-provenance` landed was the exact opposite — "by
   cwd, not by path", on the packer's old cwd-derived root — and it is retired
   rather than merely superseded: following it now steers the measurement at
   whichever tree the shell happened to sit in. Cleanliness is still tested
   per pack invocation against the resolved root, so a concurrent session
   dirtying *that* checkout mid-run still refuses, naming the root it read.

   **What the value arm is, and what it is not.** It plants a real defect in
   adopter-authored prose and asserts that some profile below the maximum catches
   it. That is a claim about **the product an adopter installs** — not about the
   residual — and this criterion once named it the residual's oracle. It read as
   one only while the smoke's main loop packed no artifact: every profile was then
   an uncovered-platform install by accident of the harness, and the accident was
   borrowed as a contract. The cost is on record. A cohort satisfied this
   criterion per member, the aggregate emptied a class, and the assertion that
   noticed was the one about the product — which then read as broken rather than
   as reporting (`port-criterion-aggregate-cost-blindness`). Two claims, two arms,
   and neither feeds the other's verdict.

   The verdict is a **price, not a screen** — the roster's opening paragraph and
   the ruling it rests on
   (TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence)
   forbid reading any criterion as an
   eligibility gate. A cohort that empties a value class still lands, carrying a
   designed answer named in its amendment: restore the class shell-side, make it
   binary-gated by a declaration the adopter receives, or accept and document.
   What a cohort may not do is land **unpriced**, and while an aggregate price
   stands unpaid a held `fail` row in `.workflow/validate-baseline.txt` is what
   keeps it visible — a machine-held record of an unpaid price. Read the file,
   not this sentence, for whether any such row stands: the `installer_smoke`
   row this clause was first written against was earned out in `97683db2` and
   reads `pass` today, so the mechanism is live and its founding instance is
   not.

   **The intersection with the measured profile's kit set became a no-op for the
   first time at §The sixth budget batch**, whose members live entirely inside the
   kit set that profile carries — so its prediction is an estimate rather than an
   upper bound, and the prediction and the measurement coincide at **three**. That
   batch also measured **both** sides, seventeen before and twenty after, rather
   than measuring one and reasoning the other, which is what makes the growth a
   subtraction rather than an inference.

   **Its honest limit, narrowed rather than deleted.** The *number* is
   machine-derived and its completeness is machine-asserted, so a cohort can no
   longer record a residual it never measured. What stays prose is the
   **judgment**: nothing forces a cohort to decide that the roster it grew is
   acceptable, because a gate that checked that would have to know what a cohort
   is, and a cohort is a queue-and-amendment concept the gate layer does not
   carry.

   **A cohort can grow the roster by zero, and the lifecycle-kit cohort is the
   worked case — recorded here because the reason is a property of the criterion
   rather than of that cohort.** Measured against the post-cohort registry from a
   clean checkout of the cohort's own commit, the binary-less leg reports the same
   **seven** omitted members the ERE cohort recorded, and none of the ten new
   descriptors is among them. The reason is the **install disposition**, not the
   narrowness of the profile the leg installs: every member of that cohort is
   `never` or `on-surface`, and neither is seeded into a freshly initialised
   consumer's registry (§The install disposition). A member no `init` registers
   cannot be a member an artifact-free `init` loses, so the leg — whose assertion
   is over the set *that payload dispatches to a binary* in the consumer's own
   registry — correctly reports no growth. The judgment the criterion leaves to
   the cohort: acceptable, and with its limit stated rather than banked. The
   instrument measures the **install-time** roster, so an adopter who later brings
   an `on-surface` member's surface into existence on a binary-less host does lose
   it, and receives it declared. That is the same shape as the ERE cohort's
   profile-scoped limit reached through a different door, and it is what a cohort
   of `on-surface` members should expect to read: a zero here is a real
   measurement about `init`, never a claim that the members are free.

   **And a cohort can grow it, which the comment cohort is the first to do — the
   same instrument, the opposite reading, and the install disposition again what
   explains the number.** Measured against the post-cohort registry from a clean
   checkout of that cohort's own commit, the binary-less leg reports **ten**
   omitted members where the two cohorts before it reported seven. The growth is
   **three** against four ported members, and the missing fourth is not an
   accounting error: `check-deprecation-task` is `on-surface`, so no `init` seeds
   it and no artifact-free `init` can lose it, while its three `zero-config`
   siblings are seeded and therefore lost. A cohort's growth is the count of its
   **seeded** members rather than of its members — the same predicate that made the
   lifecycle cohort's growth zero, read in the direction that grows.

   **The fourth budget batch priced this criterion at a width where a member-count
   reading would have been badly wrong, and the number it produced corrected the
   method rather than the batch.** Measured against the post-batch registry from a
   clean checkout of the batch's own commit, the binary-less leg reports
   **fourteen** omitted members. The batch ported eight; its growth is **two**,
   because six are `on-surface` and no `init` seeds them, while its two
   `zero-config` members are seeded and therefore lost. That is the same
   disposition predicate the lifecycle cohort's zero and the comment cohort's
   three already turned on, read a third time.

   **The absolute number diverged from the amendment's prediction, and the
   divergence is in the baseline rather than in the price.** The prediction was
   the comment cohort's *ten* plus this batch's two. The pre-batch tree already
   stood at **twelve**: increments landed between that measurement and this batch
   and none re-measured. Establishing that took a derivation — the `zero-config`
   `.gate` members the measured profile's kit set ships — validated by
   reproducing the leg's fourteen member for member on the post-batch tree, then
   run again on the pre-batch tree for its twelve.

   **So the residual is a *standing quantity*, not a per-cohort delta, and reading
   one cohort's recorded number as the next one's baseline is the error this
   paragraph exists to end.** Every worked instance above records the standing
   number, which is right; what is not right is subtracting a later standing
   number from an earlier one and calling the difference a cohort's price. A
   cohort can honestly predict **its own growth**, from the install disposition of
   its own members, and that prediction held here exactly. The absolute number is
   measured on both sides or it is not compared at all.

   **The fifth budget batch's measurement corrected the growth predicate, and the
   correction is the same shape as the one above.** Measured the same way — the
   binary-less leg against the post-batch registry, from a clean checkout of the
   batch's own commit reached by path — it reports **seventeen** omitted members,
   against fourteen before the batch. The amendment predicted a growth of *at most
   five*, the count of the batch's `zero-config` members, and the measured growth
   is **three**. The bound held and the estimate did not, for a reason worth
   stating: **the residual is measured over one profile's kit set, not over the
   registry**, so a member's install disposition is necessary and not sufficient —
   it must also ship in the measured profile. Two of the batch's five `zero-config`
   members belong to kits that profile does not carry, so no `init` of it seeds
   them and none is lost. The reading to carry forward is that a cohort predicting
   its own growth prices its `zero-config` members **intersected with the measured
   profile's kits**, and a prediction that skips the intersection is an upper bound
   rather than an estimate.

   **It was reproduced rather than taken on the leg's word**, on the method the
   paragraph above established: the `zero-config` `.gate` members under the
   measured profile's kit roots enumerate to exactly seventeen, member for member.
   The judgment is the standing one, **accept and declare**, on the terms already
   ruled above.

   **`shell-gate-tail-port` is where the residual reaches the whole roster, and it
   is the first cut whose measurement comes back as a FAILING leg rather than a
   number.** Measured the same way — the binary-less leg from a clean checkout of
   each side, reached by path — the pre-unit rev reports **24** omitted members and
   exits clean; the post-unit rev **fails**, on
   `run-gates: scripts/gates.list names no gates`. The growth is the predicted
   **two**, and the two are `check-install-disposition` and `check-shellcheck`, the
   unit's only `zero-config` members. What no prior cut met is what those two
   *were*: the `prose` profile's seeded roster is **26 members on both sides**, and
   before this unit exactly two of the twenty-six were still `.sh`. They were the
   entire battery a binary-less adopter still received, so the residual moves from
   24 of 26 to **26 of 26** and the install has nothing left to run. The refusal is
   the runner's documented one at exit 2 (§run-gates), so the leg is reporting
   correctly rather than breaking.
   **It is not a property of the measured profile and re-scoping the leg cannot
   move it**: this unit leaves no `.sh` gate declaration anywhere, so every
   profile's artifact-free install seeds an empty registry. The leg's own
   re-scoping clause asks for the profile the binary-gated class empties, and
   every one of them does.
   **The judgment is *accept and declare* with the price recorded UNPAID —
   lead-ruled at build, 2026-08-24, and the ruler is named because the label is
   what tells a later session who may revisit it.** It was the lead's to rule
   rather than the operator's precisely because it is **derived from this
   criterion rather than invented at the cut**. Every prior
   entry above closes with *accept and declare* on the ground that the adopter
   *"receives the omission declared in their own `gates.list` rather than as a
   broken battery"*, and that ground is measured false here — so the standing
   judgment is not inherited on its usual ground but reached on the rules this
   criterion already states: a cohort emptying a value class **lands anyway**, and
   what it may not do is land **unpriced**. The measurement supplies the price;
   this criterion supplies the disposition. **The mechanism is the held `fail` row** named in
   this criterion's own paragraph above, and it is the **validate stage's to
   write** — a row encoding a judgment is not a build-stage surface, and the
   governed record is here rather than in the file. The consequence a later
   session must not misread: validate meets a **failing `installer_smoke` against
   a `pass` baseline**, and that is the held row working rather than a red to
   repair. Repairing it would reverse this ruling **by mechanism** rather than by
   argument, which is exactly what a held row exists to prevent.
   **Three successors were refused, each with its ground, so none is
   re-proposed.** Making an empty registry read green is refused outright: the
   `names no gates` refusal exists to stop a vacuous pass, and turning a
   fail-closed into a fail-open on the one tree with no other signal is the worst
   trade available — being the smallest diff is not a mitigation. Holding a member
   back on shell reverses the 2026-08-23 operator ruling, and a price higher than
   predicted is an argument about **cost**, never grounds to reopen a closed
   ruling. Publishing more targets is real and is outside this criterion. What
   **is** carried forward is the fourth: re-scoping what the binary-less leg
   asserts about an all-omitted install, so the assertion becomes *the registry
   declares its omissions and, where nothing survives, says so*. That changes what
   an adopter is told they receive, which is why it is a scoped unit of its own
   and not a cut-time edit.

   **The measurement also cost more than the number.** The leg refused to run at
   all until a defect it surfaced was repaired (§check-gate-exemption-tasks): the
   instrument was sound and the *product* was not, which is this criterion's
   instrument-before-measurement ordering arriving from the direction it does not
   name. A session that reads a red instrument as a broken instrument would have
   waived the measurement and shipped the break.

   The **judgment** this batch's amendment owed is ruled **accept and declare**, on
   the comment cohort's terms and refusing its two rivals for the same reasons: an
   adopter on an uncovered platform loses each ported member and receives that
   omission declared in its own `gates.list` rather than as a broken battery;
   restoring the class shell-side reinstates the duplication the port deletes,
   which enforcement-first ranks below removal; and a binary-gated declaration is
   what the omit path already is. The honest limit rides with the ruling — this is
   a real subtraction for an uncovered host, it lands because the 2026-08-09
   directive ports the whole corpus, and it shrinks as targets are published
   rather than being repaired by the batch that caused it.

   The **judgment** that cohort's amendment owed is ruled **accept and declare**.
   On an uncovered platform an adopter loses the governed comment surface
   entirely — the comment tier, the pointer resolution and the `TODO(task:)`
   liveness together — and receives that omission declared in its own `gates.list`
   rather than as a broken battery. The two rivals are refused with cause:
   restoring the class shell-side reinstates the exact duplication the cohort's
   criterion-6 discharge deletes, which enforcement-first ranks below removal, and
   a binary-gated declaration is what the omit path already is. The honest limit
   rides with the ruling rather than behind it — this is a real subtraction for an
   uncovered host, it lands because the 2026-08-09 directive ports the whole
   corpus, and it shrinks as targets are published rather than being repaired by
   the cohort that caused it.
6. **Its corpus derivation is self-contained**, unless the duplication the port
   creates is machine-held. **The roster for this criterion is derived too, and
   by the same tool criterion 7 uses**: `bash gate-sdk/bin/run-gates.sh --emit
   port-blockers --group` partitions the still-shell members by derived corpus derivation
   (§port-blockers), so *which members share a derivation* is read off a run
   rather than answered by hand, per member, at cohort-cut time. What the arm
   emits is a decidable partition plus a counted remainder; the *unless* clause
   below stays a judgment no column reports. Found at re-selection, one step
   earlier than criterion 5: `check-spec-fence-balance`, which the amendment named, derives
   its corpus from a config-driven shell derivation and is not `mode=staged`, so
   a ported form must re-implement that derivation with nothing holding the two
   copies together. `check-action-pinning` was selected instead.

   The qualification the clause carries — *unless the duplication the port
   creates is machine-held* — is what admits a shared-library corpus call like
   `gate_find`, and the distinction is not the presence of a shared call but
   whether a machine notices when the two sides diverge.
   `check-spec-fence-balance`, the counter-example the criterion was written
   against, is a *config-driven* derivation with nothing holding its two copies
   together at all.

   **For a bridged knob the criterion is discharged by construction, which is
   stronger than the qualification asks.** The config bridge (§lib/gate.sh)
   carries the *resolved* value of a kit knob across the dispatch seam, so there
   is exactly **one** place that value is computed — the kit's shell library —
   and the binary holds no default to drift. The qualification is satisfied in
   its strongest form: the duplication is not machine-held, it is *absent*. That
   is why the crate's prune-dir default and the unit test holding it equal to
   `lib/gate.sh`'s were deleted rather than extended when the bridge landed
   (§Meta-gate conservation for the binary substrate, the
   `check-knob-default-coupling` row). The departure from the ruling that framed
   this as "a parity discharge holding the Rust default to the shell default" is
   recorded rather than quietly taken, because it is an improvement and a later
   reader must not restore the parity test as a missing piece.

   **The qualification's *other* disposition, worked — and the two are stated
   together so neither reads as the general rule.** The bridged-knob case above
   satisfies the clause in its strongest form, by making the duplication *absent*;
   the `spec_comment_surface` cohort reached the same form by the other road, its
   primitive's caller set emptying at the port so the shell original was deleted.
   Neither road is available where **live shell consumers survive the port**:
   queue-kit's `lib/queue.sh` still has seven, so its `queue_live_slugs` and its
   section regexes are permanently dual-implemented and the criterion's *unless*
   is what admits them. What discharges it there is an executed cross-substrate
   comparison rather than a deletion — one canned corpus fed to both holders, their
   classification of it compared byte for byte, run by the owning kit's scenario
   runner (queue-kit/SPEC.md §lib/queue.sh). Two facts a later port should read off
   this pair: the disposition is chosen by **whether the shell caller set empties**,
   not by taste; and *machine-held* means a standing oracle, so a parity proof taken
   once at port time satisfies criterion 2 and never this one — it expires at the
   next edit to either side, which is precisely the failure the clause names.

   **`gate_staged_matches` is the second worked instance of that road, and it
   arrived from the opposite direction — a port *creating* the twin rather than
   inheriting one.** §run-gates rules `--for` "identical to the generated hook's
   staged-path matching", and the hook's copy is `bin/gen-pre-commit.sh`'s
   verbatim awk splice of that function's body. Moving `--for` into the `--run`
   arm made the matcher two implementations with live consumers on both sides,
   which is the *unless* clause exactly. The discharge is the same standing
   comparison: one canned corpus of glob/path pairs fed to `gate_staged_matches`
   and to the crate matcher, verdicts compared byte for byte, run in the owning
   kit's fixture lane. **A second pair rides the same lane for the same reason** —
   the front-end's binary-less dispatch loop against the arm (§run-gates), one
   hermetic registry, both transcripts required byte-identical. Read together with
   `lib/queue.sh` above, the pair says the road is available to a duplication a
   port *creates*, not only to one it finds.

   **`ek_pid_alive` and `ek_lock_read` are the third instance, and they carry the
   correction a reader most needs: the *unless* clause binds on a helper **set**,
   not on the one helper an amendment happened to name.** Porting
   `check-producer-liveness` was scoped against the pid predicate alone, because
   that is the helper whose surviving shell caller is visible at a glance
   (`bin/run-validate.sh` asks it whether a lock's holder is alive). Reading the
   caller set rather than the amendment found the *reader* dual too — the same
   script calls `ek_lock_read` twice — so the discharge covers both. The lane is
   `evidence-kit/gate-tests/evidence-lib-parity.test.sh` and it takes the same
   shape as the two above: one canned corpus, classification compared byte for
   byte, no committed golden. What it adds to the pair is **a coverage assertion
   on its own corpus** — each branch the comparison is bought for is grepped out
   of the shell side, so an agreement over a corpus that classifies nothing cannot
   pass for a hold. The lesson to carry forward is procedural: before taking this
   road, enumerate the shell callers of every helper the ported member touches,
   because the disposition turns on whether *that* set empties.

   **A dead twin is deleted, not held**, and the same enforcement-first ordering
   decides it: where a shell helper has no caller and its compiled counterpart is
   live and tested, a standing parity obligation gates a duplication that removal
   disposes of. That is why queue-kit's done-slug helper lives only in the crate
   (queue-kit/SPEC.md §lib/queue.sh). The bound is **undocumented surface** — a
   helper no SPEC section names — which is what separates such a helper from the
   documented globals beside it, none of which this disposition reaches.

   **Glob semantics, committed once here rather than re-decided per port.** The
   bridge transports strings and interprets nothing — it has no glob matcher,
   because bridging a knob is not deriving a corpus from one. The commitment is
   for the reader that will interpret them: a Rust glob matcher over a bridged
   knob is **`**`-capable (globstar semantics)**, matching the `shopt -s
   globstar` the shell side enables in `canon-kit/lib/spec.sh`. The evidence and
   its limit: this consumer's arrays use no `**` today, so plain-glob semantics
   would pass here — but the shell side enables `globstar`, so the config
   surface *permits* one, and a plain-glob reader would silently mis-scan the
   first consumer who writes one. Committing to the wider semantics costs
   nothing today and closes a silent-divergence class; a porting unit inherits
   the commitment rather than re-deciding it.

   **The commitment governs a glob matcher and not a predicate that matches no
   glob**, which is the distinction `check-graph`'s port had to draw: assertion
   B's `couples ⊆ trigger` test is four branches of exact-token and suffix
   comparison with one bash-pattern branch inside it, and reading this paragraph
   as a mandate to substitute a globstar matcher for the whole predicate flips
   verdicts on the live registry (§check-graph). Stated because this is the first
   ruling a porting session finds and it is the wrong one for such a reader.
7. **Its rule invokes no external program the payload does not carry.** *Found
   at first-cohort selection.* The programs the payload is entitled to assume are
   `GATE_SDK_PROGRAM_FLOOR` (§lib/gate.sh); git is on it as the one sanctioned
   exception, because it is the floor. Under the 2026-08-09 ruling a gate that
   fails this carries **the largest named piece of port work**, not a permitted
   exclusion: the dependency is designed away — embedded, replaced, or the rule
   itself changed — and that design is the porting session's, not this section's.
   A *blocker* here is therefore work the port owes, in the sense the roster's
   opening paragraph fixes for all seven; it never reads on whether a gate ports.

   **A blocker here holds a member only where the program is the rule's own
   semantic content**, and the test between the two classes is *whether removing
   the program changes the gate's verdict*. The criterion has been silent on this
   since the declarable spelling landed, which is the narrower question that
   spelling made answerable — *is this member takeable at this cut?* — and the
   silence is what let an incidental dependency be spelled as a hold.

   - **The program is the rule.** `shellcheck` decides what §check-shellcheck and
     §check-action-run-shell assert; `cargo` decides what §check-crate-arms
     asserts; the renderer decides what site-kit/SPEC.md
     §check-docs-render-fidelity asserts. **Ruled 2026-08-23 by the operator:
     such a member ports as a wrapper that spawns the program, and the program
     stays a declared dependency** — refusing at exit 2 when absent, exactly as
     the shell form does today. Designing the dependency away was the
     "sub-project" that held these four for eleven days; it was never the port's
     work, because the dependency is the rule's content and the port moves the
     *wrapper*, not the rule. The class is therefore takeable and priced, and
     `# port-until:` has no holder on this ground; what it was minted for is a
     member whose substrate the crate genuinely lacks (exception class (c)
     below). The dependency floor those programs sit outside is not widened by
     the port: a consumer without the program gets the refusal it gets today,
     and the adopter-facing residue the ruling leaves is the bootstrap alone
     (TRAJECTORY.md §The closed rulings). **The first wrapper landed with
     `check-shellcheck` in `shell-gate-tail-port`**, and what it establishes for
     the class is more than its own port: refusing at exit 2 is not by itself the
     ruling's discharge, because the exit code is the cheap half. The wrapper owes
     **message parity at the shell form's own point in the order**, and the two
     mechanisms that buy it — a PATH presence probe and a merged-stream capture —
     live in §Fail-closed contract for the whole class rather than in one member.
     **The second wrapper, `check-action-run-shell`, consumed that contract rather
     than extending it**, which is the evidence the class-level placement was
     right: it added no mechanism beyond the merged capture's exit code, needed
     because ShellCheck grades itself by one, and its own contribution was a
     refusal text and a point in the order. It also settles a second ordering the
     first wrapper could not: a member whose scan root is a positional argument
     checks that root **before** probing the program, so an absent root reports the
     root and not the linter. **The third, `check-crate-arms`, is the first with
     more than one program, and it is what shows the class rule is about the
     *program*, not about the member**: `cargo` gets a refusal arm and `rustc` gets
     none, because the member refuses without one and merely degrades without the
     other, whose absence contributes an empty field to a cache key nothing tests.
     A wrapper's declared set and its refusal set are
     therefore two different sets — the declared set is what unit test A observes,
     the refusal set is what the member actually refuses on — and the parity run
     that separates them is the constructed scenario run **per program and for the
     set** (§Fail-closed contract). Its third ordering: the corpus-presence branch
     precedes the program probe, so a tree with no crate is clean with no cargo
     installed.
   - **The program is incidental spelling.** A text utility the rule uses to
     assemble, split or order a string the port re-expresses in the target
     language — `paste -sd, -` is `.join(",")`, and the verdict is identical
     either side of the substitution. The blocker is **priced by the cut, never a
     hold**: one line of the port's own work, exactly like a criterion-4 widening.

   **The tool cannot make this call and is not asked to.** §port-blockers reports
   *which program a rule invokes*; the class is a judgment about what the rule
   asserts, which no tokenizer sees. Declaring it is therefore presence-shaped on
   the terms §check-gate-substrate-parity's assertion G already fixed for both
   fields — presence is the verdict, absence means takeable, and an **undeclared**
   member is over-counted as takeable rather than lost. The failure direction is
   unchanged by the test: a misclassified class-(i) member reads takeable, is
   selected, and the cut discovers the blocker at composition time, which is where
   the report already puts it.

   **Two refusals, recorded so they are not re-proposed as ergonomics.** *The
   program is not on the floor* is not the test — that is the **report's** test,
   and reading it as the hold's is what puts ungrounded declarations on a tree.
   And *the port is more work than an ordinary member* is not the test either:
   every criterion prices work, and pricing is not holding.

   **What it adjudicates is exactly one thing: whether the payload carries the
   program a rule invokes.** Whether the *target* of a sanctioned spawn is itself
   ported is a **cohort-composition** question and criterion 7 does not reach it —
   a rule shelling out to `bash <emitter>` clears this criterion, because `bash`
   is on the floor, however unported that emitter is. The clause is written out
   because the closing sentence above only implies it, and a reader applying the
   derived roster literally will otherwise label such a hold criterion 7 and put
   the design work behind the wrong door.

   **The worked instance.** §The consumer remainder cohort ported three
   generated-projection freshness members whose emitters were unported shell at that cut —
   `check-trajectory-fresh`, `check-value-rollup-fresh` and
   `check-docs-mirror-fresh`. All three clear this criterion, exactly as the
   clause above says, and what actually held them was the family's own sequencing
   finding, superseded by an operator ruling and accounted for at §The
   generated-projection freshness family rather than waived here. The instance is
   worth naming because it is the first time the two doors were open at once and
   the cohort went through the right one.

   **A second instance, and it is about the tool rather than the rule.** The same
   cohort's `check-docs-kit-parity` was reported `?` — undecidable — because its
   command word was an array `gate_command` populates at run time, which no
   static scan resolves. An undecidable is neither a pass nor a blocker: it is an
   instruction to trace the runtime program set by hand, which that cohort did,
   finding `{awk, bash, the binary this cohort ports into}` and clearing the
   criterion. A session that reads `?` as a hold has let the scanner's limit
   become the rule's — **and a `?` is not a declarable hold either**, which is
   where that mistake would become durable. `# port-until:` is machine-read, so
   declaring a member whose only evidence is the tokenizer's `?` writes the
   scanner's limit into a field later sessions trust; a member declares when a
   ground is stated in its own section, never when the report shrugs.

   **The roster is derived and lives in no document, including this one** —
   `bash gate-sdk/bin/run-gates.sh --emit port-blockers` reports it against the tree at the moment
   a session sequences a cohort. That is a correctness requirement, not a
   preference for freshness. A gate's requirement need not be spelled in its
   source at all: `check-docs-render-fidelity`'s is the first element of whichever
   renderer knob that run resolves to a command — the batch one where it is
   non-empty, `SITE_KIT_RENDERER` otherwise (site-kit/SPEC.md §lib/site.sh) — and
   because both are consumer config, a consumer who repoints either changes which
   external program that gate requires. Stated as *the* renderer knob because it
   is one of two: a consumer who pins only `SITE_KIT_RENDERER_BATCH` requires that
   command and never the per-document one the gate does not invoke, which is why
   the compiled member declares **both** knobs and why the earlier spelling of
   this sentence — naming `SITE_KIT_RENDERER` alone — was true of the zero-config
   program only by the coincidence that both defaults begin `ruby`. **No literal
   roster is true for every consumer**,
   so a freshness-gated copy here would be gated against *this* repo's
   configuration while reading as a general claim — a defect a stale-roster gate
   could not detect, which is why none is shipped.

   `check-action-run-shell` is named as the **worked example** rather than as the
   roster. It clears all six criteria above and requires `shellcheck` on `PATH`,
   refusing when it is absent and invoking it per extracted block. It was held on
   the reading that a compiled form would move a toolchain requirement from this
   repo's contributors onto every adopter; that reading was **wrong**, and the
   2026-08-23 ruling above retires it: the shell form already imposed the
   requirement on every consumer that registered the gate, and the compiled
   wrapper `shell-gate-tail-port` landed imposes exactly the same one — the
   substrate of the wrapper does not move the floor. The example now reads in the
   past tense, and the class it draws outlives the member
   (§check-action-run-shell records what its port cost). Recorded rather than
   deleted because every mechanical screen puts that gate *in*, and the eleven-day
   hold is the evidence that a dependency can be mistaken for a port blocker by a
   session reading the report literally.

   **`check-gate-assertions` required `paste`, and how it surfaced is the part
   worth keeping.** The program is not on the floor, so that was owed port work of
   exactly the shape the worked example above describes — a dependency designed
   away, embedded or replaced. It was **invisible for the roster's whole life**:
   the scan abandoned that declaration before reaching the call and reported the
   member clean, which is the failure mode §port-blockers now records under its
   repaired tokenizer rules. Named here rather than in a commit message because a
   member the roster reported clean and now reports blocked reads like drift
   unless the reason is written down — and because it is the standing evidence
   that a *derived* roster still has to be re-derived by a **repaired**
   derivation, never trusted because it is derived. **Its class is (ii)**: the
   invocation was `paste -sd, -`, a comma join of a sorted label set, so the
   compiled rule spells it directly and the verdict does not move. The member was
   therefore **takeable and priced**, not held — and the eighth budget batch took
   it, so the example now reads in the past tense while the *classes* it draws
   outlive the member (§check-gate-assertions records what its port cost).

   **`check-crate-arms` was ruled a different case under the same criterion, and
   the ruling is retired with the class above.** Its rule is an invocation of
   `cargo`, and it was held permanently on the reading that a gate running
   `cargo test` over the crate cannot live inside the crate it tests. The reading
   conflates the *artifact* with the *source*: `cargo test` and `cargo clippy`
   compile and run the crate's **source** afresh in a target directory the binary
   never reads back, so a stale or broken installed binary spawning them asserts
   nothing about itself — the verdict is cargo's over the tree. It ports as a
   wrapper like the three above, and it is `install: never`, so no adopter
   receives it under either substrate (§check-crate-arms).

   **The report's honest bound is its undecidable count, and the port made it grow
   until `--needs` shipped — after which it fell for the first time.** A member
   declaring through a `.gate` descriptor has no shell rule to parse, so while the
   binary could not answer it was counted undecidable rather than reported clean —
   the §check-reads-couples precedent, a root the tool cannot bound declares `?`
   rather than guessing. Every port moved a member into that count, until it
   reached **102 of 106 members scanned**: the roster was blind over more than 96%
   of the corpus it walks, which is what the arm was sequenced against and what
   building it repaired. With `--needs` consumed at the default arm's per-member
   row the same cut reported **1**, the one member whose
   command-position expansion the tokenizer could not resolve; **that member's port
   drove it to 0**, by deleting the spawn rather than by answering it. The count is
   the share of the corpus the report cannot speak for, and it is now a bound on
   the report rather than an artifact of the port. A zero here is not a claim that
   nothing is missed: the report reads a member's own declaration text, so a spawn
   reached through a shared library — `gate_authoring_tree`'s `git`, `ek_pid_alive`'s
   `ps` — is invisible to it whatever the count says, which is
   `port-oracle-corpus-narrower-than-the-directive`'s finding and not this
   criterion's.

   **Repairing the arm's consumer is what made the knob line kind readable at
   all**, and it is recorded because the roster's numbers move with it:
   `port-blockers.sh`'s `knob_program` called `_gate_knob_value`, a function
   defined nowhere in the tree, so every knob-derived requirement had always
   reported `default unresolvable` on **both** substrates — the two agreed only by
   both failing. It resolves through `gate_knob_env_one`, the per-name face of the
   bridge the dispatcher itself uses. With it live, `check-docs-render-fidelity`'s
   requirement is **measured** as `ruby` rather than predicted from
   `SITE_KIT_RENDERER`'s default, which is the class-(i) worked example reading off
   a run for the first time. The run corrected the prediction as well as replacing
   it: the resolved row cites `SITE_KIT_RENDERER_BATCH`, the knob the zero-config
   gate actually spawns, not the per-document one the prediction named.

   **The last registered member ported, and the count the *battery* is measured by
   reached zero.** `check-docs-render-fidelity` is the fourth criterion-7
   wrapper and the first whose requirement is **knob-derived**, so it is the only
   member that exercises the report's third line kind end to end — a `?<TAB><knob>`
   pair resolved through the same bridge the dispatcher uses, rather than a program
   spelled in the rule. Two facts it settles beyond itself. First, a wrapper's
   refusal is not always an `on_path` refusal: this member's shell form *probes its
   oracle by running it*, so an absent program surfaces as the probe's own exit
   status inside the member's own message, and the compiled form reproduces bash's
   127 (or 126) for a pipeline element it could not start rather than substituting
   a presence test that would print different text. Second, the ordering register
   gains its fourth entry, and the first whose probe sits behind more than one
   corpus test: the git-repository test, then the scan-root test, then the
   program. Both are
   §Fail-closed contract's to state for the class; what belongs here is that the
   ordering is still per-member and still read off the shell text.

   With it in, `--group` reports **0 still owed, 0 takeable, 106 already ported**
   and the default arm 0 undecidable — both measured rather than asserted, and both
   statements about the **gate battery** alone. The honest bound two paragraphs up
   is unchanged by that zero, and it is not the only reason the zero is no
   completion claim: the completion predicate TRAJECTORY.md rules is bounded by the
   *tree*, and the arm that evaluates it is §port-blockers' `--tree`. A registry arm
   reading zero owed says the battery is ported and says nothing about the tree —
   the confusion this section's own oracle used to make unavoidable, and the reason
   the two corpora are now named wherever either number is.

   **The fifth wrapper is the one the report could never have found, and it lands
   after the count reaches zero.** `check-producer-liveness` is one of the two
   unregistered kit-shipped members, so the arms that walk `gates.list` never
   counted it and no number of theirs moved when it ported — `--tree` reaches its
   *file*, on the terms §port-blockers records for that limit, and never its
   requirement. It is a
   wrapper anyway: `ek_pid_alive` falls back to `ps -p`, off the floor and reached
   through a **shared library** rather than through the member's own declaration
   text, which is the sharper of the two reasons the oracle is blind to it because
   it is true of registered members too. Two facts it settles beyond itself, both
   §Fail-closed contract's to state for the class. First, the ordering register
   gains an entry that is not an ordering at all: the shell form fires **no**
   refusal, so *fire where the shell form fired it* is the constraint when a shell
   refusal exists and never a licence to inherit a false green where one does not.
   Second, a wrapper's program can be a **shell builtin**, and the honest route to
   it is `bash -c` — on the program floor — rather than a second off-floor
   dependency on `/bin/kill` or a narrowing to the fallback program alone. Its
   sibling `check-surface-duplication` is the counter-case in the same batch and
   worth stating beside it: measured the same way, every program its shell form
   spawned was on the floor and **none survives the port**, so it is not a wrapper
   and its declared requirement set is empty rather than filtered to empty.

**New gates are born native by default; shell is the exception and it needs a
stated cause** — operator-ruled 2026-08-14 and re-affirmed the same day on
corrected criterion-5 evidence (TRAJECTORY.md §The closed rulings). This reverses
the prior default, under which a born-native gate was a design ruling taken per
gate. It is the only measure so far aimed at the port's *denominator* rather than
its remainder: while the port runs at two to three members an iteration, every
gate landing in shell adds one the port then owes.

**The default's domain is a tree that carries the crate the gate would compile
into** (`GATE_SDK_NATIVE_CRATE`). Where the authoring tree does not — every
vendoring consumer — shell is not an exception but the only substrate, and the
consumer-facing authoring path is unchanged. This is not a narrowing of the
ruling but the only reading a consumer can execute: an adopter receives no gate
implementation source (TRAJECTORY.md §The objectives, §Consumer payload), and
`native/` ships no `checks/` and no `smoke/`, so `gate_kit_roots` never selects
it and `init` never vendors it. An adopter therefore cannot author a compiled
gate at all, which is why `templates/check-skeleton.sh`, gate-sdk/README.md's
first-gate walkthrough and §Consumer smoke's kit-landing checklist stay exactly
as they are: a flipped walkthrough would be false on its face rather than merely
premature. What the domain decides is which surfaces the flip reaches —
publisher-facing ones (this section, and CLAUDE.md's always-loaded authoring
line) change; consumer-facing ones do not.

**A born-native gate is the member these criteria do not describe, and only some
of them bind on it.** The roster is stated over *ports* — a gate that exists in
shell and is being moved — so a gate landed as a Rust module and a `.gate`
descriptor with no shell original (canon-kit/SPEC.md §check-measured-claim was
the first, operator-ruled 2026-08-12) reads them as follows, and a later reader
must not apply the parity criterion to a member that has no second substrate:

- **1 and 3 bind unchanged.** It registers in `gates.list` and it lands in the
  generated hook, so `check-graph` proves its manifest the same way.
- **2 does not bind.** Parity is *between substrates*, and a gate with one
  implementation has none to prove against. Its `good/`+`bad/` pair is its whole
  oracle, exactly as it would be for a new shell gate — which is a weaker
  guarantee than a ported member's parity run. That observation survives the flip
  intact; what changes is the conclusion drawn from it, which is now **the
  accepted price rather than the reason to ship shell**. The trade reversed
  because its other side changed, and this is recorded rather than presenting the
  new default as self-evident: under the 2026-08-09 directive the corpus ports in
  full, so shipping a new gate in shell does not *avoid* the compiled
  implementation — it defers it, adds a parity run that would not otherwise be
  owed, and grows the denominator the directive is racing. The weaker oracle is
  paid once; the deferred port is paid again at every cohort cut that has to
  sequence it.
- **4 does not bind** where the gate's **assertion target** is not gate source —
  criterion 4's own predicate above, applied unchanged. A **conservation-table row
  may still be owed**, because `couples=` can reach a declaration-path-processing
  oracle transitively, and that is assertion C's question rather than this
  criterion's. Stated as two facts rather than one conjunction because the
  conjunction was tried and falsified: `check-measured-claim`, the first
  born-native member, scans the governed-prose surface and clears criterion 4,
  while its `couples=` names `scripts/*.sh` — an emitter that treats declaration
  paths as a set — and it therefore earns a row (§Meta-gate conservation for the
  binary substrate).
- **5 binds hardest, and it is the price.** A `.gate`-declared member is omitted
  from the `gates.list` of a consumer whose host the roster carries no artifact
  for, so on an uncovered platform a born-native gate does not run *where a shell
  gate would have*. For a port that trade is neutral — the shell form is deleted
  either way; for a born-native gate it is a real subtraction against the
  alternative of shipping shell, and it is what the operator ruling weighed —
  in its corrected form, `native/targets.list` shipping one target, so the
  uncovered set is every macOS adopter rather than a narrow hypothetical. Under
  the flipped default that subtraction attaches to **every** new gate, which is
  why it is exception class (b)'s subject below and why its accumulation *rate*
  is owned elsewhere rather than here (`born-native-omission-accumulation`). The
  growth therefore has an observer from the day the flip lands rather than being
  a change nothing watches: each born-native member adds one to the set an
  artifact-free install omits, which is exactly the roster the binary-less leg
  above derives and asserts complete at a non-zero count.
- **6 and 7 bind unchanged**, and a born-native gate is designed to clear them at
  authoring time rather than owing the work afterwards, which is most of why it is
  cheap: the substrate it needs already exists or it is not born native. Under a
  *default* that last clause would read as a licence — any gate can be declared
  un-bornable — which is exactly what exception class (c) below bounds.

**The flip is not a discount, and two of the criteria above are *more*
load-bearing under a default rather than less.** The `good/`+`bad/` pair is now
the gate's entire oracle in the ordinary case instead of the exceptional one, and
the `--reads` declaration plus assertion C's conservation row (§Meta-gate
conservation for the binary substrate) are owed at authoring time rather than
deferred to a port. A born-native gate shipping without them ships *less*
coverage than the shell gate it replaced, which is the one outcome the flip must
not produce.

**The exception criterion: two live classes, each with a stated cause, and one
retired.** Shell is taken only under one of these, and the gate's own SPEC
section states which class and why. A further class is an amendment, not a
judgment call.

- **(a) — RETIRED 2026-08-23 by operator ruling; no gate is permanently shell.**
  The class held that a gate auditing the dispatch relation — whether a gate
  declares itself, whether a descriptor and a subcommand agree — stays shell
  because a compiled form could pass *itself* with a broken binary. The grounds
  for retiring it are owned at §Meta-gate conservation for the binary substrate,
  the paragraph below its table: the shell auditor already reads the binary's
  `--list`, an absent binary is exit 2 under the fail-closed contract, and a
  stale one is §check-gate-binary-fresh's red — so the false green the class
  feared has an owner that is not the auditor's substrate. `check-crate-arms`,
  which was held beside this class on criteria 4 and 7, is retired with it
  (criterion 7's passage above). **No `# no-port:` holder remains among the
  registry's members**, and the six members these rulings held are owed to
  `shell-gate-tail-port`. The field's live holders sit outside that registry: it
  reaches any tracked script (§The `# graph:` manifest), and this tree's plain
  scripts declare on it in a count only §port-blockers' `--tree` trailer is
  authority for — the hook generator on criterion 6's single-producer rule
  (§gen-pre-commit), a consumer's measured-claim emitter on a provenance ground
  (§port-blockers), and the whole consumer-smoke class on the 2026-08-30 ruling
  at §Consumer smoke, *The port disposition*.
  Scoping the sentence to the registry is what keeps it true of a corpus that has
  since parted from the field's own.
- **(b) The gate's subject is a platform the target roster does not cover.**
  Criterion 5 omits a `.gate` member on exactly the platforms `native/targets.list`
  carries no artifact for. A gate whose findings arise *on* those platforms would
  therefore be omitted precisely where it is the only reader — the one case where
  born-native does not merely weaken the oracle but deletes it at the point of
  use. **Ends when the target lands**, so the cause names the target, and the gate
  ports under the ordinary criteria once `platform-support-ci-matrix` widens the
  roster.
- **(c) The rule needs substrate the crate does not carry, and building it is not
  that unit's work.** Criteria 6 and 7 bind on a born-native gate unchanged, so
  the bound on this class is what keeps them from reading as a licence: the cause
  **names the missing substrate and the entry that owns it**, and the gate lands
  shell inside the port corpus with its blocker already declared. That is strictly
  better than the pre-flip state, where the same gate landed shell and its blocker
  had to be re-derived by `port-blockers.sh` at some later cohort cut.
  **Temporary by construction** — it expires when the substrate lands.

**A declarable spelling for the held classes was designed, refused for want of
holders, and has since been minted — and the sequence is recorded rather than
collapsed, so a later reader sees that the field waited rather than being
reserved.** Classes (b) and (c) are temporary, so `# no-port:` must not carry them
— a marker conflating permanent with temporarily-held would drop members that will
port, replacing the remainder's over-count with an under-count. The honest
spelling is a **second** field, `# port-until: <slug>` (§The `# graph:` manifest,
which owns it): `blocked` collides with a word `port-blockers.sh` already uses for
something *derived*, while `until` is this tree's existing word for a temporary
disposition — §check-gate-exemption-tasks pairs `# until: <live-slug>` with
`# permanent: <reason>` on exactly this axis, two annotations rather than one with
two values. That precedent also answers the rot objection, and is now that gate's
own second arm: it holds the slug to a **live** queue entry, so when the blocker
lands and the slug moves to Done the gate reds and the declaration must be
dropped. The **closed-roster rule** — the field roster carries no field lacking a
named reader — is what deferred it: it would have shipped with **zero holders**,
and a field minted empty is a reservation however good its design. What supplied
them was `cohort-held-members-port-prerequisites`, whose roster of held members is
exactly the missing input; the field landed **with** its declarations in one unit,
which is the only landing the rule admits.

**The spelling's domain is wider than these two classes, and that widening is
§The `# graph:` manifest's to state**: the holders are criterion-7 blockers read
over gates already authored, fitting no lettered class, so the field's subject is
*any temporary hold with named work owed and a live owning entry* rather than the
born-native exception letters. Recorded here because this is the section that
drafted it for the letters.

**The cause is recorded in the gate's own SPEC section**, beside the rule it
governs, on the same terms every other design ruling for that gate is recorded —
never in a central roster, which would be a maintained list of the residue that
derivation-first refuses and that rots at every port. It is reachable there
because that section is written in the same unit as the gate (§Consumer smoke's
kit-landing checklist), and it has three readers rather than none: the reviewing
session, which finds it where every other ruling for that gate already lives; the
port-track selector, for which a class-(c) cause states a blocker and its owning
entry that `bash gate-sdk/bin/run-gates.sh --emit port-blockers` can derive but not attribute; and
the session landing the missing substrate or the missing target, for which a
class-(b) or class-(c) cause *is* the list of gates that become portable with it.
**A held member's cause now has a machine-read companion and the two must agree**:
`# port-until:` names the owning entry, that entry carries the disposition and
the cost, and the gate's own section carries the ground — the single statement of
that split, §The `# graph:` manifest citing it rather than ruling a second one.
A member may not declare the field
until its own section states its cause — a declaration whose ground is inferred
from a sibling gate, or from this section's shared worked-example prose, is the
one shape the field must not normalise, since the field's whole value is that a
reader reaches the ground from the declaration in one hop. That rule is
**machine-held** rather than discipline: §check-gate-substrate-parity's assertion
H opens the section the declaration's own `# spec:` field names and reds when it
does not state the hold.

**What the classes deliberately exclude**, because an exception criterion is
defined by its refusals: *the rule is easier to write in shell*, *the author is
faster in bash*, *the corpus is small*, and *it is only a temporary gate* are none
of them causes. Nor is *criterion 5 makes it a real subtraction* on its own —
that is true of **every** born-native gate, so admitting it would swallow the rule
the day it landed. Class (b) is the sharpened form of that argument and the only
form of it that survives.

**No gate enforces the flip, and the ground is cost rather than impossibility.**
The distinction is stated because the stronger claim was made and then measured
false. Three enforceable shapes have been weighed:

- **A per-gate `# substrate: shell — <cause>` header asserted over every shell
  member.** Buildable today and the shape that would work — but it demands one
  retrospective declaration per unported member, sixty against this tree at the
  flip, each deleted again as its member ports. It becomes cheap when the residue
  is small, which is the condition to revisit it under.

  **`# no-port:` is the same shape over a different subject, and it does not
  discharge this** — stated because a green §check-gate-substrate-parity must not
  be read as having landed born-native enforcement. This header would enforce the
  **born-native default** — *may this gate land in shell at all?* — over newly
  authored gates; that field reports the **port remainder** — *will this gate ever
  leave shell?* — over existing ones. Different subject, different corpus,
  different reader, which is also why the field is not simply named
  `# substrate:`. It is likewise why that field mints **no value vocabulary and no
  `candidate` default**: a value asserted over every still-shell member reinstates
  exactly the retrospective sweep costed above, thirty-odd declarations each saying
  nothing and each deleted at its port, where default-by-absence buys the same
  derivation for three lines.
- **A baseline roster of today's shell members that a gate diffs against.** A
  maintained roster, which derivation-first refuses, and it rots at every cohort
  cut.
- **A git-anchored discriminator** — the same header, demanded only of a member
  whose declaration file entered the history after the flip's own commit. This
  shape was first refused on the ground that **no** discriminator exists while the
  port runs, a newly authored shell gate being indistinguishable from one still
  awaiting its port. Measured at the merge, that ground is false:
  `git log --diff-filter=A` dates every declaration file, this repo already checks
  out at full depth because `check-trajectory-fresh` reads the commit graph, and
  the scan costs about a second over the whole check corpus. Needing no
  retrospective sweep, it is strictly cheaper than the first shape. What it costs
  instead is three things rather than one impossibility: a rename re-dates a file,
  so the plain form false-positives — five of seventy-two paths on this tree, all
  resolved by `--follow`, which is a heuristic and not a guarantee; the anchor is
  a literal commit no derivation produces; and as weighed it is **publisher-local**,
  because in a vendored tree every declaration file was added by the vendoring
  commit and the assertion would redden a consumer's whole registry. That last
  cost is smaller than it looks, the default's domain being exactly the
  crate-carrying tree, but making the shape consumer-safe is design work it has
  not had.

The flip therefore rests on this section and on CLAUDE.md's always-loaded
authoring line, and the enforcement disposition is **filed rather than
flagged-and-skipped** per the gap-disposition rule — the unit that landed the flip
did not open the work of landing a gate, and which iteration builds one is not
this section's to rule. Enforcement-first is not waived: it ranks a gate above
discipline *where a gate is available*, and the shape now known to be available is
unbudgeted rather than unavailable.

### The first cohort, and the rule that selects the next

**Two members: `check-action-pinning` and `check-action-gh-repo`** — one cohort
rather than two coincidences, and the pairing is the whole reason. They share one
corpus derivation byte for byte (§check-action-gh-repo: *"The walk is
§check-action-pinning's"*), read no knob between them, and declare the same
single unbounded walk root. The substrate work is therefore done **once** and
proved **twice**. `check-action-pinning` needed no new implementation at all: the
crate already carried its rule as the `reference-only` disposition, kept live so
the substrate would stay exercised until a port needed it, and this cohort is
what that disposition was held for.

**The exclusions are recorded, because the next selector meets them again.**
`check-memory-off` fails criterion 2 decisively: its `--fixture <dir>` arm is a
different code path from its live arm, and the live arm's corpus is not in the
tree at all — it is the harness memory directory under `HOME`. A gate whose
fixture arm bypasses the derivation being ported has no parity oracle for the
part that matters, which is criterion 4's self-referential-parity hazard arriving
through another door. (It also word-splits an override into multiple globs and
reads four knobs — the weaker ground, recorded second so resting on it does not
leave the stronger one unwritten.) **That exclusion is spent**: §The third budget
batch took the discharge it named — the redundant arm deleted, the pair
re-pointed onto the knobs, and a constructed scenario over the residue. The
record of why it stood stays, because what changed is that the discharge arrived,
not that the ground was wrong. `check-action-run-shell` is the near miss and
is criterion 7's own case, above.

**The next cohort is the largest set of criteria-clearing gates sharing one
corpus derivation** — an ordering rule, not a bound: the 2026-08-09 ruling ports
every gate. **The instrument that makes the rule applicable is
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group`** (§port-blockers): it partitions
the still-shell members by derived corpus derivation, largest group first, with
the mechanically derivable criterion columns beside each member — so "largest",
"criteria-clearing" and "sharing one corpus derivation" are each read off a run
rather than off a session's reading of the tree. **What the tool means by "one
corpus derivation" is not obvious from the phrase and is specified at
§port-blockers** — a two-factor key, neither of which is `couples=`; read it
there before reasoning about why two members did or did not group. Its output is
advisory and its key is deliberately not fused with `couples=`, so a group whose
members' corpora visibly diverge is a **finding the selecting session
adjudicates**, never a cohort the tool cut. A cohort that retires a **blocker** several later cohorts are queued
behind outranks a larger one that retires none, which is the exception §The POSIX
ERE matcher works and `check-roadmap-fresh`'s hold worked before it. **Where
neither of those selects, the increment composes by budget** — a third arm, and
what makes it usable is the precedence rather than the arm alone. The arm: *N
criteria-clearing members, taken as N independent units*, sized against the
iteration's own budget. Precedence across the three is total and is read off a
run rather than off a preference. The size arm selects first; the
blocker-retiring override outranks it wherever a blocker several members are
queued behind can be retired — the keyed-knob increment is its second live
instance and its smallest, a pair of members in separate kits behind one
wire-format change, which is the shape the override is *for* rather than a weak
case of it;
the budget arm is reachable **only** when a `bash
gate-sdk/bin/run-gates.sh --emit port-blockers --group` run reports no takeable group — so its
precondition is a verdict from the instrument this section already names for the
size arm, never a session's reading of the tree.
**The size arm is now permanently exhausted rather than exhausted at a cut, and
that is a property of the rule rather than a reading of one remainder.** A
`--group` run over the owed members partitions them into groups of exactly one,
so no group has a second member to amortize a walk across — and the born-native
default forecloses a new one forming, because a gate written native was never in
the shell partition to join a group. A later selector therefore reads the arm's
precondition off the run as before and should expect it to keep reporting the
same verdict; what would reopen the arm is a *consumer* contributing shell gates
that share a derivation, not this tree.
**The takeable tier reopened at the eighth cut and the eighth cut emptied it
again, and both readings are oracle reads rather than numbers this section
holds.** Run on 2026-08-22 after two holds were retired — one criterion-4 bind
that was never a hold ground and one criterion-7 blocker of the
incidental-spelling class (§The port-candidate criteria owns both rulings) —
`bash gate-sdk/bin/port-blockers.sh --group` trailed *106 member(s) scanned, 2
group(s) formed, 0 undecidable, 98 already ported and excluded, 3 permanently
shell and excluded, 3 temporarily held and excluded; 5 still owed, 2 takeable at
this cut*. That superseded the reading §The sixth budget batch recorded at its
own cut, where the tier was exhausted and the budget arm had nothing to compose
from. **The eighth budget batch then took both**, and the same command re-run at
its build cut trails *106 member(s) scanned, 0 group(s) formed, 0 undecidable,
100 already ported and excluded, 3 permanently shell and excluded, 3 temporarily
held and excluded; 3 still owed, 0 takeable at this cut* — the owed count down by
two and the takeable tier empty, so every unported member again sits behind
`cohort-held-members-port-prerequisites`. **The 2026-08-23 ruling then emptied
both excluded tiers at once** — no `# no-port:` and no `# port-until:` holder
remains **in the registry this arm walks**, the tree corpus `--tree` walks being a
different set whose own holders only that arm's trailer counts — and the same command trailed *0 permanently shell and excluded, 0
temporarily held and excluded; 6 still owed, 6 takeable at this cut*, the six
owed to `shell-gate-tail-port`. **That unit then took all six, and the arm's
trailer reaches the floor**: re-run at its closing cut, the same command trails
*106 member(s) scanned, **0** group(s) formed, 0 undecidable, **106** already
ported and excluded, 0 permanently shell and excluded, 0 temporarily held and
excluded; **0** still owed, 0 takeable at this cut*. Every field of that trailer
is now a zero except the scan itself, which is the arm reporting that it has
nothing left to compose rather than failing to compose — the reading a later
session must not mistake for a broken run. None of these readings is this
paragraph's to
hold: a later session re-reads the trailer, because a dated read is evidence that
the arm's precondition **can change**, never the tier's current value. Through
both readings the groups were of exactly one, so the size arm's verdict is
unmoved and the budget arm is the only composer left. Shared derivation is the axis
**while it selects**, because it is what made this cohort
cheap: the walk is ported once and proved N times, and the parity comparison is
over one corpus shape rather than N; the budget arm is what this section says
when that axis stops selecting. Selecting by kit, by profile, or by whatever
is easiest next all re-import work this cohort only paid once. The gates failing
criterion 4 or 7 are **sequenced last and budgeted for**, because each carries a
design problem — a self-referential parity oracle, an external program on `PATH`
— that the port owes an answer to rather than a waiver. Neither is ported and
patched later; both are designed, then ported.

**The eighth budget batch is `check-gate-assertions` and `check-tree-terms`, and
it is two batches of one member in every sense that matters.** The oracle read
above is what selected them: the size arm is permanently exhausted, so the budget
arm composed, and the takeable tier at that read *was* exactly these two. Neither
was held — both had their `# port-until:` declarations retired on 2026-08-22 with
their prices relocated to their own sections: §check-gate-assertions priced
`paste -sd, -` as criterion 7 class (ii) and named the GNU-awk three-argument
`match()` a capture-API re-expression, and §check-tree-terms priced its
criterion-4 bind as a fixture widening before the port. Each carried its own
descriptor, its own registry entry, its own widening and its own parity run, and
no comparison spanned both — the property the paragraph below states in general.
**Both bound criterion 4** and each paid it the same way, by widening its pair
first; `check-gate-assertions` also ended a *contingent* immunity deliberately and
is now self-auditing, which its own section owns. Two facts about the remainder
are worth carrying, both measured at the build cut rather than predicted here:
`paste` left the battery's program set **entirely**, on a re-run of `bash
gate-sdk/bin/port-blockers.sh` where the only programs any rule still names are
`shellcheck`, `cargo` and the renderer; and the residual `gawk` floor lost
`check-gate-assertions`, the holder the probe covered, while
§check-docs-render-fidelity still held it (§check-gate-assertions records the
probe, its scope, and what is deliberately *not* changed on the published
requirement). That last holder left with `shell-gate-tail-port`'s port of it, so
the residue is now **empty by measurement** — which is the precondition
`interpreter-floor-gawk-residue-empty` was filed to wait for, and still not the
narrowing itself.

**Criterion 5's price for that batch, measured on both sides — and the detour it
took getting there is the part worth keeping.** Both members are
`# install: zero-config` and both live in `gate-sdk`, which the measured profile
carries, so the predicted growth is **two** with the profile intersection a no-op.
`installer_smoke`'s binary-less leg, run from a clean checkout of each rev reached
by path, reports **twenty-two** omitted members before the batch and
**twenty-four** after — the prediction exactly, and both sides measured rather
than one measured and one reasoned. The **judgment** is the standing one, **accept
and declare**, refusing its two rivals for the reasons already ruled: an adopter on
an uncovered platform receives each omission declared in its own `gates.list`
rather than as a broken battery, restoring the class shell-side reinstates the
duplication the port deletes, and a binary-gated declaration is what the omit path
already is.

**The post-side reading was blocked first, and what blocked it was worth more than
the number.** The smoke failed at its first profile: the compiled gate binary
carried its dependencies' build paths and `init` commits that binary into the
consumer's tree, so §check-tree-terms' newly honest binary arm reddened a
consumer's own first battery. That is the leak guard working on its first live
encounter, and the measurement is what found it — a cohort's criterion-5 price is
paid on a real install, which is precisely why this criterion refuses a reasoned
number. The defect was fixed at its source rather than exempted (§build-native),
after which the leg ran and returned the twenty-four above. The lesson for a later
cut: an instrument that will not run is a finding about the product, not a licence
to reason the number out.

**"Never as one cohort" is the property that makes a budget batch safe, and it
reads as bookkeeping only until that is said.** A batch's members carry **no
joint proof**: each takes its own `.gate` descriptor, its own registry entry, its
own fixture-pair parity run and its own live-tree and edge-root arms, so no
shared walk is claimed and no comparison spans two of them. What follows is the
property worth having — **dropping a member mid-batch invalidates nothing**. A
member that turns out dearer than it was sized leaves the batch and the rest land
unchanged, where a cohort losing a member loses the amortization it was composed
on and is re-planned. A batch is therefore never recorded, argued or merged as
one unit of work; the plural is the mechanism rather than the wording.

**The batch size is a budget, and the budget is deliberately not a knob.**
Config-via-env exists for configuration that varies **by consumer**; a batch size
varies **by iteration**, against that iteration's other work, so it is a judgment
the composing session makes with the queue in front of it. A knob would freeze a
default that reads as the answer, and the next session would size against the
default rather than against its budget — the very defect the arm was minted
against, reintroduced as configuration and harder to see for being declared.
What the sizing session weighs instead, stated once so it is not re-derived: the
per-member inputs `--group` prints beside each member — `lines=`, the
declaration's own line count, and the mechanically derivable criterion columns —
plus its own reading of each member's declaration, the iteration's non-port work,
and the fixed per-iteration ceremony.

**`lines=` is a floor on a port's size and never a ranking of it**, and the
evidence is two consecutive cuts rather than a caution. §The fifth budget batch
records that its cost "was in *interfaces*, not in logic" — a ranking of logic
could not see six command-line arguments — and names
`check-template-copy-parity` as a member whose cost is concentrated where the
line count hides it; that member was then the **cheapest by line count** of the
cut that took it, so the second cut attests what the first found. Cost behind a
**spawned** tool is invisible to the column on the same terms (the
spawn-invisibility rule below). So the count is one input beside the criterion
columns and the session's own reading, never the answer: a session that reads
the column as an answer stops looking for the cost the column cannot see.

**A budget batch records only findings here, never a member roster.** Every
cohort section below earned its place by having a shared derivation to explain,
and a budget batch has none, so its default record is **nothing**. The rule: *a
budget batch adds a section to this SPEC only where it has a finding to record* —
an adjudication a later selector would otherwise re-make, a primitive it landed,
a criterion it discharged. Membership and progress are **derived** instead: a
ported member leaves a `.gate` beside its deleted `.sh`, and the count is
`scripts/measured-claims.sh`'s `ported-gate-members`, which §check-measured-claim
already holds to the tree. Without the rule the section count scales with the
tail — roughly one section per batch across the remaining singletons — and the
fixed per-iteration ceremony the arm exists to amortize gets paid a second time,
in prose, on the one surface a porting session must read end to end.

**The arm names no remainder, which is what keeps it kit content.** It is stated
generically — no gate names, no member roster, no count of any tree's remaining
corpus — so a consumer who ports gates receives the mechanism and not this
project's work queue. A kit literal naming one tree's singleton set would publish
that queue as everyone's mechanism, the same defect the `check-graph` rule-content
split exists to prevent (§check-graph).

**Two other homes were weighed for this arm and each lost on a stated ground**,
recorded so the placement is not re-opened. **lifecycle-kit's scope template** owns
the general economic-composition test, and a port-specific selection arm placed
there would bind every consumer's iteration including the consumers who port
nothing, and would make the general iteration contract carry a term only this kit
can define; the reach it would have been bought for is already bought, because a
consumer's port entry names this section as canonical for every increment, so a
scope session composing a port iteration reaches the arm along the path it walks
anyway. **The consumer's ruling record** (TRAJECTORY.md here) lost by its own
contract rather than on a preference: it records rulings the operator closed and
never authors one, and its pointer-first convention registers a ruling at its
mechanism's canonical home — which is this section. This section already owns the
two arms the third joins, it is the surface a selecting session already loads,
its reach is exactly the port and nothing wider, and it is kit content, so the
arm ships as mechanism to every consumer who ports gates.

**The arm's producer and consumers, and the verdict that it adds no field.**
Producer: the session composing a port iteration — scope when it ranks the queue,
and the authoring stage when it writes the batch amendment — triggered by a
`--group` run whose verdict is *no takeable group*. That trigger is live rather
than test-only, being the same run the size arm already reads at every cut.
Consumers: that same session, by reading this section; and the iteration lead at
batch-cut, which reads a batch amendment's per-delta work-class labels and needs
the arm to know that a batch's deltas are independent and may be cut apart where a
cohort's may not. **There is no new field**, and saying so is the point rather
than an omission: a later reader must not go looking for a machine reader that was
never intended. The arm is an authoring contract like the two it joins, neither of
which has a machine reader either, and §check-queue-entry-budget's own
*the gate cannot hold this* paragraph is the house form for saying so.

**A primitive's *remaining* consumers are derived, never recorded here.** Each
cohort section below records the members it landed, which answers what was ported
and not what is left — and the leftover is the number a session sizing the next
cohort actually needs. It is one command, because a ported gate leaves a `.gate`
descriptor beside its former script: the still-shell consumers of a shared
derivation are the `*/checks/*.sh` files naming it that have **no** sibling
`.gate`. Deriving it beats a roster that would rot at every cohort
(derivation-first), and stating the command once is what stops each sizing
session re-inventing it. **`--group` is the whole-corpus form of the same
doctrine, not a rival instrument**: the one-liner above answers *what is left of
one named primitive*, and `--group` answers *how the whole remaining registry
partitions* without being told a primitive to ask about. A session that already
has a primitive in hand uses the one-liner; a session choosing which primitive to
take next runs the arm.

**What this cohort landed.** Both rules ship as
compiled subcommands, proved byte-identical against the shell gates on each
fixture pair, the live tree and the edge roots — the comparison run **while both
implementations still existed**, which is the only order in which parity can be
proved at all, since §check-gate-substrate-parity assertion A forbids a
descriptor and a script coexisting in one dir. The **descriptors are live**: both
`.gate` files sit in `gate-sdk/checks/`, both members are registered in this
repo's `gates.list`, and the two shell scripts they replaced are deleted. **The
port is therefore proved and live**, and every consequence that follows from a
live descriptor has arrived with it — the dogfood settlement bites
(§What the dispatch seam does not settle), the stale-binary path is armed
(§check-gate-binary-fresh), and the disclosure claim is false and corrected.

**The premise the hold rested on was half wrong, and the correction shipped with
the descriptors rather than after them.** The hold's ground was that a vendored
`.gate` with no binary behind it reds a freshly installed consumer's battery and
that the first tag publishing binaries is what clears it. The first half was
true. **The second half was false**: both binary meta-gates derived their trigger
by globbing `*.gate` across the resolve dirs and read no registry, so no tag and
no registry edit could have cleared the red in a tree that legitimately has no
binary — most sharply every adopter on `init`'s own omit path, whose members are
dropped from the registry while the descriptors still vendor. The repair is the
load-bearing predicate §check-gate-binary-fresh states — a declaration is not a
dispatch — and it is what made these descriptors landable at all. Criterion 5's
own reasoning is unaffected and stands: a prebuilt binary is still how a
*dispatching* consumer gets one.

**The predicate was necessary and not sufficient, which is the part a later port
most needs.** It clears every tree that *declares* without dispatching. A tree
that legitimately **dispatches** and has no binary is a different problem and it
had a real instance: `run-consumer-smoke.sh` vendors kit roots by copy, and
site-kit's `smoke/install.sh` registers both members of this cohort because it
installs the workflow template they lint. No predicate can make that green — a
registered dispatch with no binary is exactly the harness error `gate_command`
exists to raise. That harness therefore receives the built artifact now
(§Consumer smoke), and `check-gate-binary-fresh` left its scratch registry,
because a consumer tree does not verify a build it did not make. A port that
lands descriptors checks both halves: which trees *declare*, and which trees
*register*.

**The second cohort was ruled by kit, and both selectors picked the same set** —
recording why is what keeps the ordering rule above usable rather than quietly
overridden. The operator ruled 2026-08-11 that the next cohort is **queue-kit,
taken as a whole kit**, under TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence.
The rule above warns in the same breath that selecting by kit re-imports work a
cohort only pays once. It does not here, because queue-kit is the kit whose corpus derivation *is*
the queue file: read off the `# graph:` manifests, its members couple
`TASK-QUEUE.md` alone but for two — `check-roadmap-fresh`, which adds the roadmap
projection and the queue config its own `trigger=` names, and
`check-queue-slug-liveness`, which names `kit:*.sh` only as a reverse trigger, the
shape the disposition table already records for it (§Meta-gate conservation for
the binary substrate). Every member is `dir=one valve=none tier=precommit`, so
criterion 3 is clear kit-wide and a green `check-graph` after the port is
end-to-end proof the manifest survived the substrate change. The kit boundary and
the corpus boundary coincide **here**; a later selector must not read this as
licence to take a kit whose gates share nothing.

**Two members are held on shell, and the ground is sequencing rather than
exclusion.** The whole-kit ruling was made on the premise that every member
cleared the criteria the existing substrate answers, and that premise turned out
false for two — each ruled held by the operator the same day, `.sh` unmodified,
with its port work named and owed:

- **`check-roadmap-fresh` — cohort composition. SPENT 2026-08-18, its port
  landing with its own emitter in one commit; the record of why the hold stood is
  kept because it is what the sequence was waited on.** It invoked `bash` on
  `queue-kit/bin/roadmap.sh --emit`, one of the `lib/queue.sh` consumers the
  originating port did not touch (queue-kit/SPEC.md §lib/queue.sh), so nothing in
  that cohort ported the emitter it shelled out to. **The 2026-08-11 hold stood on
  that ground and was relabelled, never lifted** (operator-ruled 2026-08-12):
  the label read `criterion 7`, and criterion 7 clears here — `bash` is
  on `GATE_SDK_PROGRAM_FLOOR`, which is why `bin/port-blockers.sh` reports none of
  the generated-projection freshness gates — while what the hold actually turns on
  is whether the cohort ports the emitter, which that criterion explicitly does not
  adjudicate. A hold is **per-member**, keyed on *is this gate's emitter ported?*

  **The design it owed is answered, so what remains is sequence alone.** The three
  candidates — shell out to the emitter, reimplement the emission format in Rust
  beside the surviving script, or collapse the emitter itself onto the binary —
  are adjudicated: the third is the one that pays, and a ported emitter is a
  **non-gate arm** with a stated contract (§The non-gate arm). `roadmap.sh` ports
  by applying that ruling unchanged. This is a **retirement of the hold's reason,
  not a reversal of the hold**: the member is still unported, and it is now held
  by nothing but its place in the queue. **That place arrived**: the arm and the
  gate landed together, which is the pairing the hold existed to preserve — the
  adapter's never-disagree guarantee is satisfied on the other substrate rather
  than repealed, and porting exactly one of the two is what would have spent it.
- **`check-queue-prose-precondition` — an ERE engine. RETIRED, its port landing
  in the twelfth cohort below; the grounds are kept because they are what the
  engine was sized against.** It does not *transport*
  `QUEUE_KIT_PRECONDITION_REGEX` across the bridge, it **interprets** it, and the
  knob is consumer config carrying an arbitrary POSIX ERE; its `awk` also runs
  `gsub` with alternation, groups and negated classes, so porting it means an ERE
  engine — hand-written here, on the span semantics §The POSIX ERE matcher fixes
  rather than on any bar against depending on one. Its
  own `gsub` runs over a pattern baked literally into awk source, which a port
  hand-compiles, so the substitution engine an earlier sizing put inside this hold
  is outside it (§The POSIX ERE matcher). Sizing a subset to this consumer's one
  configured regex is **foreclosed** by the argument criterion 6 already makes for
  globs: the config surface permits what this consumer happens not to write, and a
  narrow reader would silently mis-scan the first consumer who writes it. The
  engine had landed, so what this member was held on was its own port — and that
  is the port the twelfth cohort took.

Neither hold is an eligibility screen, and citing it as one inverts the rule §The
port-candidate criteria denies in its opening sentence and TRAJECTORY.md restates.
A criterion a member fails **orders** the work; the 2026-08-09 directive ports the
whole corpus over time, so holding a member for a later cohort is the sequencing
those criteria prescribe — the same treatment criterion 7's own worked example
gives `check-action-run-shell`, which it names the largest piece of port work
rather than a permitted exclusion.

**The criterion-7 claim that helped select this cohort was false, and its fixture
pair could not have caught it.** The cohort was taken partly on the reading that
`git` is the only external program across the kit; the `roadmap.sh` shell-out
above falsifies that, and it survived design review because
`check-roadmap-fresh`'s `good/` and `bad/` cases both pass pre-baked files that
steer the assertion off the live emitter. A ported member's pair would therefore
have gone **green over an arm with no implementation** — the same vacuity the
`check-gate-output` disposition exists to close, arriving through a different
door. A cohort is sized off what its members *execute*, never off what their
fixtures reach.

**The generated-projection freshness family, derived per member rather than
labelled.** Its members byte-compare a tracked projection against a live
`bash <emitter> --emit`: `check-footprint-fresh`, `check-trajectory-fresh`,
`check-enforcement-fresh`, `check-value-rollup-fresh`, `check-docs-mirror-fresh`,
`check-roadmap-fresh`. **The family is closed 6/6.** On the per-member key the
relabel above fixes — *is this gate's emitter ported?* — no shell emitter is left:
footprint, enforcement-map and the value-rollup join went first, and
`scripts/gen-docs-mirror.sh` (127 lines), `drift-kit/bin/trajectory.sh` (242) and
`queue-kit/bin/roadmap.sh` (76) followed as the `docs-mirror`, `trajectory` and
`roadmap` arms, taking 445 lines of shell out of the tree. Each is a non-gate arm
of the binary (§The non-gate arm) landed with its comparator, so **every** member
now compares against an in-process function call rather than a spawn. That is
where this family's `bash` hop is actually retired rather than merely relocated,
and the retirement is now literal: `fresh.rs`'s `emit`, `emitter_path` and its
`[[ -x ]]` probe lost their last caller with the tail triple and were deleted.
What differed per member is what clearing the hold *cost*:

| Member | Beyond the byte-compare | What it owes past its emitter |
|---|---|---|
| `check-footprint-fresh`, `check-trajectory-fresh`, `check-enforcement-fresh` | nothing | nothing — a spawn and a string compare. All three **fully discharged**: gate and emitter both ported, compare in-process, no shell left. `check-trajectory-fresh`'s emitter came last, and paid one extra: `date -d` over git's own `--date=short` output was a day difference dressed as an epoch subtraction, so it became a civil-date helper |
| `check-value-rollup-fresh` | marker-block extraction | nothing — the block grammar is the projection's, not a corpus derivation. **Fully discharged**: the join ported with it, and the block reader it owned moved to the shared marker module beside the writer that module gained (§lib/inject.sh) |
| `check-docs-mirror-fresh` | an orphan sweep, its own walk over `<root>/docs` | the walk (the crate already carries it) **and** a fail-closed repair: the sweep silences stderr, so an unreadable tree reads as no orphans. **Both paid** at §The consumer remainder cohort. **Fully discharged**: the emitter followed, paying `realpath -m --relative-to` as crate path arithmetic |
| `check-roadmap-fresh` | a second assertion over `TASK-QUEUE.md` through the shared roadmap adapter | a **criterion 6** answer — the roadmap arm calls that same adapter, which is what makes queue-kit/SPEC.md's *"the emitter and the gate can never disagree"* true; porting the gate alone would duplicate it with nothing machine-held. **Fully discharged**, the two landing in one commit so the guarantee never lapsed |

**Whichever member of this family ports first owes the report cap, and it is
settled rather than left to that session** — the renderer the crate carries is
uncapped and the `head -20` above is the caller's, so the cap lands as one crate
constant with its first reader in that same commit (§The diff renderer). **§The
consumer remainder cohort discharged it**, landing the constant with the two
readers this family had for it.

**Three members ported ahead of their emitters, and the accounting is written
here rather than left implied.** `check-trajectory-fresh`,
`check-value-rollup-fresh` and `check-docs-mirror-fresh` ship as subcommands from
§The consumer remainder cohort, under the operator's remainder ruling. The two
sentences above — *not one of them is ported, so every member is held* and *a
ported byte-comparator spawning a shell emitter buys nothing against dual
maintenance* — are **true and are not repealed**. What changes is their standing
for these three: they are a *sequencing* finding, and the standing rule at
TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence is that no
port-candidate criterion survives as an eligibility gate, so the technical
problem a criterion names is work the port owes rather than an exclusion it may
take. The operator's remainder ruling is the later and more
specific instruction. **The honest number for those three was therefore zero at that cohort**: the
dual-maintenance win was nil until their emitters followed, and what the cohort
actually banked for them was *consumer-tranche completion* — a gates directory
with no check script left in it — never a shrunken interpreter surface. Written
down because the next selector reading three ported members as evidence that this
family's hold was wrong would be reading the opposite of what happened. **The
emitters have since followed and all three are off zero**, which is the shape of
the correction rather than a repeal of the reasoning: the number moved because the
work moved, exactly as the figure was defined to.

**Across the whole family the count is six, and it belongs beside that zero
rather than appended somewhere else.** `check-footprint-fresh` was the first to
land its comparator *and* its emitter in one unit; enforcement-map and the
value-rollup join followed, then the `docs-mirror`, `trajectory` and `roadmap`
tail. For every member the spawn is gone, the shell is deleted, and the
dual-maintenance win is banked rather than owed. This is a live count of members
whose win has actually banked, and porting an emitter moved it — the property
that made the figure worth stating at all, and the reason it is now the family's
own size rather than a fraction of it.

**The emitters were filed, not adopted, and the file is now drawn down to
nothing.** Porting them was real and identified work sitting outside the remainder
cohort's ruled scope, so it became a costed queue entry under scope-gated intake
rather than something pulled in mid-cohort. Three of the six were drawn from that
entry as one unit and the remaining three as the next; the entry closed with
them.

**The transferable conclusion re-points the ordering rule.** The cheap cohort in
this family is **the emitters, not the gates**: a ported byte-comparator
spawning a shell emitter removes no shell, so it buys nothing against the
dual-maintenance ground the port rests on
(TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence), and of the
three candidate designs the `check-roadmap-fresh` hold names only *collapse the
emitter itself onto the binary* pays. The selection rule above — the largest set
of criteria-clearing gates sharing one corpus derivation — mis-selects this
family, because what its members share is a **spawn shape** rather than a corpus.
Recorded as a worked limit on that rule, not as a change to it.

**And a carried claim about their fixtures is corrected here, because a later
selector would inherit it.** The vacuity above was generalised as *all six steer
their pairs off the live emitter through the emit-source positional*; it is
true of **five**. `check-docs-mirror-fresh` takes no emit-source argument — its single
positional is `[root]`, both case arg files are `.`, and it drives the live
generator against the synthetic case tree, so its pair already proves the
emitter-executing arm and the warning does not apply to it. For the other five
the warning stands unweakened, and their parity must be bought by a live-tree run
or a constructed scenario — criterion 2's `# no-fixture:` treatment, reached
through a bypass arm rather than through an absent pair. **All five have
since attested it**: `check-trajectory-fresh` and `check-value-rollup-fresh` at
§The consumer remainder cohort, and the tail triple's emitters by a
transition-scoped byte-compare against the shell form over the live tree, held
while both implementations existed.  `check-roadmap-fresh`'s pair is the sharpest
instance of the warning — a ported pair would have gone green over an arm with no
implementation at all — which is why the parity run and not the pair is cited as
its emission's proof. The attestation sharpens the claim in one place the original
wording left soft — a live-tree run must be a **stale** one, because a clean
comparison exercises neither the diff renderer nor the cap, and those are the
parts a pair steered off the emitter was never going to reach.

**Two orthogonal nine-of-ten splits existed and neither predicted a held member.**
One is the walk axis — every member reads fixed paths but `check-queue-slug-liveness`,
which walks a bridged glob array. The other is the fixture axis —
every member carries a `good/`+`bad/` pair but `check-task-conservation`, whose
`# no-fixture:` reason is structural (queue-kit/SPEC.md §check-task-conservation).
Both were drawn before the criterion-7 and ERE probes, and both land on members
that **did** port. The transferable lesson is not the arithmetic: a third and a
fourth axis existed and nobody had looked down them, so a nine-of-ten on any one
axis is not a difficulty estimate.

**One shared mechanism this port discharged for a later cohort, and that debt is
now closed.** The canon-kit `spec_manifest_files` callers owed two: basename-glob
list matching beside the crate walker's extension filter, and a Rust
`gate_kit_roots`. `check-queue-slug-liveness` needed the first and it landed here,
inside the crate's single sanctioned walk implementation so the recorder still
observes it (§Meta-gate conservation for the binary substrate). **The second
landed with the canon-kit cohort itself** — as a bridged resolved value rather
than a re-derivation, since the fallback predicate is anchored at the shell
library's own location and a binary the installer copies elsewhere cannot recover
it (§lib/gate.sh). Neither is owed by a later selector.

**The sixth cohort is lifecycle-kit taken as a near-whole kit — ten members on
the `lib/stages.sh` derivation, operator-ruled 2026-08-13 and delivered whole.**
`check-stage-skill-coverage`, `check-skill-binding`, `check-lifecycle-registration`,
`check-gap-inbox-neutrality`, `check-merge-attrs`, `check-stage-evidence`,
`check-lesson-disposition`, `check-survey-record`, `check-shim-restatement` and
`check-scratch-citation` ship as descriptors dispatching to the binary; the ten
shell scripts they replace are deleted. Two members stay shell and their grounds
differ in kind. **`check-stage-entry` was held, and the hold is retired**: it reads
`LIFECYCLE_KIT_PREDECESSOR` by key, and the bridge carried scalars and indexed
arrays only, so the hold turned on the wire format growing keys — a prerequisite it
shared with `check-evidence-baseline`, which is why the queue held one entry for it
rather than one per member. The keyed-arm increment paid exactly that (§lib/gate.sh)
and both members ported together; the grounds are kept because this section is
canonical for every cohort's holds *and their disposition*.

**Splitting that increment was refused, and the grounds generalise.** Authoring
found a second missing channel of the same class (§check-reads-couples' filter
field) affecting only one of the two members, and the cheap move was to deliver
the unaffected member and re-block the other. Two of the four grounds are ones a
split cannot answer: the increment's selection rode the **blocker-retiring
override on the pair**, so delivering one member undercuts the ground of its own
selection; and splitting would leave the keyed wire with **no non-empty live
instance in this tree**, since only the predecessor map actually resolves at
every pre-commit and only if its reader ported — a wire format whose in-tree
exercise is the empty case is one whose first real consumer finds the bugs. The
other two: both gaps are one defect class — *a wire the compiled substrate
cannot say what the shell substrate could* — and holding a member on a
designed-but-unbuilt prerequisite buys delay and retires no risk. The honest
cost is recorded with it: the increment ran larger than it was scoped, by one
wire-format change scope did not see.
**`check-close-surfaces` is out rather than held**: it sources
no `lib/stages.sh` at all, so it is *unsized*, and a later selector owes it a
sizing rather than inheriting a hold whose ground was never established. **The
twelfth cohort below is that later selector, and the sizing it owed is recorded
there with its cause.**

**The kit and corpus boundaries coincide again, and the sharing claim is weaker
than queue-kit's — stated so a later selector does not inherit an overstated
precedent.** Only five of the ten call a `lifecycle_*` function; the other five
source `lib/stages.sh` **for its knobs alone**. What the ten share is therefore
the **config surface**, not a corpus walk. That is still the right axis, and the
reason is criterion 6: all fifteen knobs the ten read are defined by
`lib/stages.sh`, each crosses the bridge as a value the shell library computed
once, and the crate holds no default for any of them to drift from — so the
substrate work is paid once and discharged ten times **by construction** rather
than by ten parity tests. The library's own load-time config validation rides the
same way: it runs shell-side during knob resolution, so a compiled member inherits
the malformed-config refusal without a Rust twin. A selector reading the by-kit
warning above should read this as its second worked exception, on a config surface
where queue-kit's was on a corpus.

**What this cohort paid that no previous one did is not the gates. It is their
callers**, and that is the transferable part. Five cohorts deleted a
`checks/<name>.sh` and left every caller undisturbed, because every caller was the
fixture harness and that harness has resolved substrate-agnostically since the
first cohort. lifecycle-kit is the first kit whose **scenario runners**, whose own
**`bin/`** and whose own **`smoke/`** name their gates by script path — including
`bin/enter-stage.sh`, the stamp mechanism every stage entry in a consumer running
the lifecycle goes through. Sizing such a cohort as N gate ports and meeting the
callers at implementation time is exactly the failure criterion 7 exists to
prevent, so a selector reaching a kit that ships `bin/` or `smoke/` enumerates the
callers **before** the cohort is cut. The conversions themselves are
lifecycle-kit/SPEC.md's — §Testing for the runners, §bin/enter-stage.sh for the
tool layer.

**`check-lifecycle-registration` is a freshness gate in miniature, and the
freshness family's hold key does not reach it.** It byte-compares tracked text in
the agent file against a live derivation, which is structurally what the six
generated-projection members above do — and those are held per member on *is this
gate's emitter ported?*. The difference is exact rather than a judgment call: the
family's members shell out to `bash <emitter>`, while this member's emitter is
`lifecycle_registration_block`, a **library function it sources directly**, and
that library is the derivation this cohort compiles. The emitter therefore ports
*in this cohort*, in the same motion as the gate that reads it, so the hold's
condition is satisfied rather than waived. Written out because a selector applying
the family's rule mechanically would hold this member, and the ruling did not.

**Two knobs this repo's own configuration hides from a live-tree parity run**, and
the canon-kit cohort's lesson — the live tree is the oracle a session trusts most
and it proves nothing about a branch it does not execute — applies twice here.
`LIFECYCLE_KIT_SESSION_BOUNDARY` is set to `iteration`, which disables
`check-stage-evidence`'s cross-stage session-id distinctness map, so every
live-tree run takes the disabled branch. `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`
carries no `**`, so `check-scratch-citation`'s globstar path is never exercised
live even though criterion 6's glob commitment binds the compiled reader to it.
Both were proved by constructed scenario instead, and a third property narrows
every parity run in this tree without being a branch at all: this repo runs a
six-stage machine, so `LIFECYCLE_KIT_STAGES` and `LIFECYCLE_KIT_PREDECESSOR` are
both overridden and no run here reads the kit default.

**One rule was implemented rather than transliterated, and one diagnostic had to
be built.** `check-merge-attrs`'s shell form reaches for three `comm` invocations
over six process substitutions; the compiled form implements the **set difference**
the contract states, which is not locale-dependent where `sort`'s collation is —
the kit-roots cohort's general ruling, applied verbatim.
`check-lifecycle-registration`'s stale-block report is `diff`'s **normal format**,
which no previous cohort had needed: it is an LCS walk inside the member, because
the crate's one-spawn-site rule (§Fail-closed contract) leaves no way to shell out
for a diagnostic. `check-stage-evidence`'s missing `fail_closed` guard, the one
shell defect this cohort found, is **dissolved rather than repaired**: the
compiled member reaches no subprocess at that site, so the failure mode is absent
by construction. Recorded so the port is not read as having silently dropped an
assertion, and so nobody opens a repair against an absent file.

**The seventh cohort is the `spec_comment_surface` family — four members on one
corpus primitive, operator-ruled 2026-08-13 and delivered whole.**
`check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness` and
`check-deprecation-task` ship as descriptors dispatching to the binary; the four
shell scripts they replace are deleted. It is the first cohort ruled **knowing it
was design-then-port rather than cheap**: all four fail criterion 4 — they audit
comment content on governed sources, and a governed source is what a gate
declaration is — and that criterion's remedy is a designed answer rather than a
waiver. The design is recorded where it generalises rather than here:
§Fixture-pair discipline owns the widened-corpus rule and criterion 4 owns the
discharge and the live-tree demotion that comes with it.

**What the cohort paid, and it is the corpus rather than the rules.** The four
members' `good/`+`bad/` pairs carried `.sh` and `.md` sources and nothing else,
across all eight case dirs, while the primitive spans four arms — `*.sh`, the
`*.gate` descriptor, `*.rs`, and the workflow directory's tracked tier. The
parity instrument therefore covered **one arm in four**, and the three it missed
were precisely the three that make criterion 4 bind. Widening the pairs came
first and the port second; the same vacuity shape this section already recorded
once — *"a ported member's pair would therefore have gone green over an arm with
no implementation"* — is what it was bought against.

**Criterion 6 is discharged in its strongest form, and this is the cohort that
empties a primitive.** `spec_comment_surface`, its `_with_templates` twin, the
whitelist predicate and `spec_queue_slugs` had exactly these four members as
their callers, so the shell forms are **deleted rather than duplicated** — the
duplication is absent rather than machine-held, the same disposition the config
bridge earned and for the same reason. That is the opposite verdict the same
criterion earns on a library with live consumers, and both are recorded so
neither reads as the general rule (canon-kit/SPEC.md §lib/spec.sh states the
price: three documented names a consumer's own gate could have called, answered
by shadowing the gate). `check-spec-pointer`'s criterion-6 hold is discharged by
that removal; `spec_manifest_files` stays dual, because members outside this
family still call it, which is the pre-existing duplication the canon-kit cohort
accepted and this one does not re-open.

**A trigger gap the earlier widening left behind is repaired in the same unit.**
The primitive gained its `*.gate` and `*.rs` arms when the conservation contract
landed and **no member's `couples=` followed it**: against this tree that left
every descriptor and every crate implementation source inside the corpus and
outside the trigger, so staging an edit to a ported gate's descriptor or module
re-ran none of the four gates that read it — and the port makes it worse in the
one way that matters, by moving the four members' own declarations into that
class. All four manifests gain `*.gate` and `*.rs`, and two spellings widen from
`.workflow/*.txt` to `.workflow/*`. Both added tokens are **bare globs**: a token
naming this repo's crate directory would publish one consumer's layout into a kit
file and be false for every other (CLAUDE.md §The provenance seam), where the
bare glob matches the corpus derivation exactly and over-selection in a
trigger-shaped set is cheap by design.

**The eighth cohort is the first the selection rule chose rather than a reader**,
and its own section is where that matters: §The canonical-spec
`spec_canonical_specs` cohort records the members, the group key, the group size
and the undecidable count from the `--group` run, the two larger groups it
rejected and on what authority, and the price. Two facts belong here rather than
there, because they are about the **rule** and not about that cohort. A group's
key being non-empty does not make it a shared derivation — the run's largest group
shares only the fail-closed guard, which derives no corpus. And the tool's columns
are not the whole screen: the group it ranks second contains a member
§Meta-gate conservation for the binary substrate holds on shell, which no column
reports and which a selector applying the run mechanically would have taken.

**The eleventh cohort exhausts the size arm a third consecutive time, and what
that streak means is a fact about the rule rather than about any cohort.** §The
consumer remainder cohort takes the whole remainder of this repo's own gates
directory — ten members — under the documented blocker-retiring override, with
the `--group` verdict at its cut recorded there. Three consecutive selections
made without the size arm is the point at which the arm stops being the ordering
rule's live half here: what actually orders the remaining corpus is which
*blocker* a cohort retires, and the size arm survives as the tiebreak it was
always the general form of. Two grounds compound in this instance and are
separated so neither is read as the other. The first is the override's own — the
tranche retires `consumer-gate-port-disposition`, an entry several cohorts have
demoted rather than closed. The second is **amortization**, and it is new: the
first-mover design for a consumer-declared member (§check-gate-substrate-parity's
assertion-B owner clause) was paid by the first tranche, so every later member
inherits it and marginal cost per member sits at its floor. Amortization is not
a licence to take a large set whose members share nothing — it lowers the price
of a cohort the override already selected, and does not select one.

### The first budget batch

**Composed by budget rather than by group — the arm's first live selection.**
`--group` reported 37 groups over the remainder with no group takeable: 34
singletons, two pairs each held on shell for a stated ground, and the one
7-member group already ruled out. Six members were taken by a criteria audit
over the three smallest-key groups, on the budget the composing session set.
Per the property above, the batch is not one unit of work and was never
merged, recorded or argued as one: each member took its own descriptor, its
own registry entry, its own parity run and its own deletion, in six
independent commits. This section records the two findings the batch
produced and the one verdict its cut owed — never a member roster, which the
rule above already gives a derivation for.

**A missing shared primitive is invisible to `--group`, and a later selector
must not read the run's silence as clearance.** `--group`'s key is shell
libraries and globs; two of the six members hand-rolled the identical
heading-bounded section walk while sharing no shell library, so nothing in
the run's own columns names the primitive they were about to duplicate a
third time. `native/src/section.rs` lands it, and what this section owns of it
is the **contract**: the primitive yields a bounded line range and nothing
else, so a caller reads no heading level and no heading text. The matching and
closing rule that *produces* that range is stated once, as a directive at the
implementation itself, and is deliberately not restated here — a second copy is
what rots, and the one drafted here had already drifted from the source it
described before this section was a day old. The seam holds at the argument
boundary — the crate ships the walker and no section vocabulary, every name
arriving as a caller argument.

**Two members sharing that primitive is sequencing, not a rejoined cohort,
and the axis is stated so a later selector does not merge their proofs.** The
cohort axis above keys on a shared *corpus derivation*, proved once and
compared over one corpus shape N times. `check-brevity` and
`check-doctrine-registration` share a *parsing primitive* instead and walk
different corpora — `CLAUDE.md` against `CLAUDE.md` plus
`doctrine-kit/DOCTRINE.md` — so there is no single corpus shape to compare
over and neither's parity run lends the other proof; each kept its own. The
sequencing is soft in both directions, which is what kept the batch's
drop-any-member property true even for the two that share code: dropping
`check-doctrine-registration` would have left the primitive with one caller,
still correct, and dropping `check-brevity` would have moved the primitive's
landing into the surviving member's own unit.

**The substrate-sensitive set was re-derived at the batch's cut, and the cut
adds no member.** A port moves declaration paths — `<kit>/checks/<name>.sh`
becoming `<name>.gate` — which can move other members into or out of the
derived set, so assertion C was run fresh after all six descriptors landed
rather than inherited from an earlier cohort's table reading. None of the
six reaches a gate declaration path through its own `couples=`:
`scripts/git-hooks/*`, `.claude/agents/*.md`, the two named delegation-kit
files, `CLAUDE.md` plus `doctrine-kit/DOCTRINE.md`, and `.workflow/*,.gitignore`
each name a corpus outside `<kit>/checks/`, `native/src/gates/*.rs` and
`native/src/*.rs` alike. §Meta-gate conservation for the binary substrate
gains no row from this batch.

### The second budget batch

**Composed by budget again, and the precondition held through an adjudication
rather than on the run's face.** `--group` reported one non-singleton group over
the remainder, and it was a **phantom pair**: its other half was a member §The
port-candidate criteria's exception class (a) then ruled permanently shell (a
class since retired), whose permanence the tool could not see. Every group was in fact a singleton, the size
arm was exhausted, and the increment belonged to the budget arm — the hand
adjudication §The first cohort, and the rule that selects the next sanctions, *an
advisory group is a finding the selecting session adjudicates*. The batch is not
one unit of work and was never merged, recorded or argued as one. Under the arm's
record-only-findings rule this section carries the two adjudications the cut
produced and no member roster: membership is derivable from the tree and the
count from `scripts/measured-claims.sh`.

**Criterion 3 does not bar a `tier=commit-msg` member, and the criterion's
literal value is a proxy rather than a bar.** It reads `tier=precommit`, and its
stated reason is that the member *lands in the generated hook, so a green
`check-graph` after the port is end-to-end proof the manifest survived the
substrate change*. A `commit-msg` member lands in the generated **`commit-msg`**
hook, which `check-graph` holds against `--emit-commit-msg` on identical terms,
so the criterion's purpose is met in full — the batch's first member is where the
literal value and the purpose come apart. **The dispatch and the config bridge
were already there, and the assumption ran the other way first:** the generated
`commit-msg` hook emits `run_gate <name> <declaration>.sh "$msg_file"`, which
reads as a shell-only path with no `env` bridge and would owe a new emitter arm.
It does not — that argv is built through the same `command_rel` → `gate_command`
path the pre-commit emitter uses, and `gate_command` is what emits
`env <bridged knobs> <binary> <name>` for a `.gate` member. One emitter, one
resolver, both tiers; the message-file positional lands after the subcommand name
and reaches the gate module's argv slice unchanged. Recorded because a member the
criterion's literal text appears to bar, and does not, reads as a rule being bent
unless the reason is written down.

**Criterion 4 was failed and discharged by fixture pre-work, and the order is the
whole discharge.** The batch's second member scans every kit's `checks/`
directory and its own declaration path lies inside it — criterion 4's predicate
exactly. **Widen first, then port.** The discharge is a fixture corpus carrying
every arm of the derivation being ported, and the pair did not: probed directly,
every kit in both trees shipped only `.sh` files under `checks/`, so the `.gate`
arm the shell already implemented correctly was exercised by no case at all. The
pre-work is fixture construction with no design in it — a kit whose `checks/`
holds a descriptor, and a **mixed** `.sh`+`.gate` kit, which is what proves the
union of the two globs rather than either alone. **The live-tree arm is demoted
from proof to smoke for such a member**, on the terms this section's criteria
already set for a gate-source auditor: assertion A forbids a descriptor and a
script coexisting in one resolve dir, so the comparison necessarily runs on the
pre-descriptor tree — a corpus the port then changes. Its verdict is recorded as
**no disagreement found on the pre-descriptor tree**, never as parity proved; the
edge-root arm keeps its own separate value, and here it earned it, catching a
path-construction disagreement no relative-root case could reach.

### The third budget batch

**Composed by budget again, and the precondition read clean on its face this
time.** `bash gate-sdk/bin/port-blockers.sh --group` read every group a singleton
at the 2026-08-18 cut, which is the budget arm's stated precondition, so the
increment is a hand-composed batch rather than a cohort. The batch is not one
unit of work and was not merged, recorded or argued as one: each member took its
own descriptor, its own registry entry, its own parity run and its own deletion.
Under the arm's record-only-findings rule this section carries the cut's findings
and no member roster — membership is derivable from the tree and the count from
`scripts/measured-claims.sh`.

**A member was admitted on the wrong criterion, and correcting the record is the
first finding.** The prior cut admitted `check-memory-off` on a criterion-7
argument: its `jq` use is a path query the already-ported `check-settings-pins`
performs on the same grammar, so the design criterion 7 demands is paid. That
argument is true and it answers a criterion this member was never held on — §The
first cohort, and the rule that selects the next and §The settings cohort, and the
crate's first dependency both hold it on **criterion 2**, and the latter
adjudicates the point directly: *its blocker is the oracle, not the dependency*.
Retiring `jq` could therefore not retire the hold. Recorded because a member
admitted on the wrong criterion reads, to the next selector, as evidence that the
criterion was satisfied.

**The second code path criterion 2 named was redundant, and nothing in the record
had noticed.** The shell form's live arm resolved three paths from three knobs;
its `--fixture <dir>` arm resolved the same three from a directory. **Every path
the fixture arm redirected, a knob already redirected**, so the arm bought a
shorter spelling and paid for it with the divergent code path that was the whole
of the criterion-2 finding. Deleting the arm is what dissolves the hold rather
than working around it, and it is flagged as an interface removal rather than
folded into an implementation note: `--fixture` was a documented flag on a
governed surface. The pair is **kept** and re-pointed onto the knobs — a port that
had dropped it along with the arm would have found that out at
`check-gate-fixture-coverage` — and it stops being vacuous, because it now drives
the code path the live battery drives.

**The residue took the constructed scenario, and its verdict is recorded here
because a verdict that outlives its session is a verdict nobody wrote down.** The
one derivation no in-tree case can reach is the **default** memory dir, which the
harness names under `HOME` from the repo toplevel. A throwaway `HOME` with that
layout beneath it, both implementations run over the same state, bytes and exit
codes compared: **nine cases, byte-identical on stdout, stderr and exit code** —
memory dir absent, holding only `.gitkeep`, holding a regular file; local settings
absent, on-pin, off-pin, and explicitly `null`; and a word-split multi-glob
override in a clean and a polluted state, which is the "weaker ground" the earlier
record placed second and which no single-dir case reaches. **One arm was not
compared and is recorded as retired rather than proved equal:** the shell failed
closed when a local settings file was present and `jq` absent, and the ported
member reads JSON itself, so that branch has no counterpart to be equal to.

**Re-using the crate's JSON layer exposed two semantic divergences, and both are
named rather than discovered later.** `native/src/json.rs`'s `Path::compile`
accepts every construct the pins grammar permits, but re-use is not substitution.
*(a)* Comparison becomes **structural**: the shell compared `jq -c`'s compact
output against the manifest's right-hand side by raw string equality, so `1` and
`1.0` compared unequal; the manifest declares expected JSON rather than an
expected byte form, and `check-settings-pins` already reads it that way, so
keeping string equality would have left this member and its sibling disagreeing
about one grammar.
*(b)* Null handling is **opposite**, and `settings_pins.rs`'s branch must not be
re-used verbatim: `check-memory-off` silently skips a null actual, because a null
means the local file sets no override for that key, while `settings_pins.rs`
treats a null as a fail-closed absent pin — correct for the tracked file it reads
and a correctness regression here, since it would red every tree whose local
settings simply omit a pinned key. The ported member re-uses `Path::compile`,
`eval` and `values_equal` and supplies its own null disposition; the scenario
carries a case for it, so the distinction is checkable rather than asserted.

**The jq dividend, and the boundary no reader may take it past.**
`port-blockers.sh`'s criterion-7 report over the scanned members named
`check-memory-off` the battery's only remaining `jq` consumer, so the port
subtracts `jq` from **the gate battery's** dependency floor, against
TRAJECTORY.md objective 1. It retires `jq` from nothing else: `installer/lib/`
shells to it on the shipped install path and refuses naming the program where it
is absent (installer/README.md §Requirements), and guard-kit, the delegation-kit
templates, drift-kit and `scripts/` carry their own uses. *"The batch retires
jq"* is false in every direction but the battery's — §The settings cohort's
honest-claim paragraph is the model this one is written to.

**A member that produced no finding still owes its reckoning, or the batch's
silence reads as coverage.** `check-root-tiering` ported with none: every
primitive it needs was already in the crate, and its hermetic pair carries its own
allowlist, scan root and both positionals, so unlike its batch-mate's it is a
genuine parity oracle and the port needed no scenario beside it. Its seam holds
too — the built-in fallback set is generic orientation plus two configured knobs,
so nothing consumer-shaped crossed into the crate, and the allowlist proper stays
optional consumer config on the `graph-vocab.sh` pattern.

**One consumer shelled to a deleted path directly, and no listed gate would have
caught it.** `context-kit/smoke/agents-md.sh` invoked `check-root-tiering.sh` by
literal path against a *second*, unrelated throwaway repo — found by grepping the
tree rather than by a red, since neither `check-docs-cmd` nor `check-md-refs`
scans a `smoke/*.sh` script. The failure mode would have been a hard
"No such file or directory" crash the first time that smoke ran, not a
diagnosable message. Both call sites re-point onto `gate_command`, resolved
against the vendored tree rather than the throwaway repo's cwd. Recorded because
the general lesson is not about this smoke: **a port's Point-5 reader enumeration
covers gates, and a direct shell-out from a non-gate script is outside every one
of them.**

**Assertion C was re-run fresh after both descriptors landed**, on the rule §The
first budget batch established — a port moves a declaration path, which can move
*other* members into or out of the derived substrate-sensitive set, so the reading
is never inherited from an earlier cut. Verdict: §Meta-gate conservation for the
binary substrate gains no row, neither member's `couples=` reaching a gate
declaration path.

### The fourth budget batch

**A wide cut — the operator ruled 6–8 members against the prior cuts' two to
three, and the width is inside the arm's proven envelope rather than a
relaxation.** §The first budget batch took six and established the
drop-any-member property at that width. The batch is not one unit of work and
was not merged, recorded or argued as one: each member took its own descriptor,
its own registry entry, its own parity run and its own shell deletion, in its own
commit. Under the arm's record-only-findings rule this section carries the cut's
findings and no member roster — membership is derivable from the tree and the
count from `scripts/measured-claims.sh`'s `ported-gate-members`.

**The size arm is permanently exhausted, and that is a change to the rule rather
than a reading of one cut.** `bash gate-sdk/bin/run-gates.sh --emit port-blockers --group`
partitions the owed members into groups of exactly one, 0 undecidable — no group
has a second member to amortize a walk across — and since the born-native default
no new gate can arrive to form one. The budget arm's stated precondition, a
`--group` run reporting no takeable group, is met by that run.

**What is new is the selection ground.** §The first budget batch chose from the
three smallest-*key* groups, a shared-derivation proxy this partition does not
offer.
This cut composes by **declaration size plus criterion cleanliness**: the cheap
band is the members at or under ~103 shell lines with a fixture pair, a
generated-hook tier, no criterion-7 blocker and no design fork. **The honest
limit rides with it**: cheapness is not a shared derivation, so the batch
amortizes nothing across its members and per-member cost is unchanged from taking
them one at a time. The whole economy is **session overhead** — one cut, one
criteria audit, one assertion-C re-run, one criterion-5 residual measurement, one
amendment — spread over the width. That is a real saving and it is the only one;
a later selector must not cite this batch as evidence that width is cheap in
itself.

**Half the band binds criterion 4, and the ordering is the discharge.** Every
port moves a declaration path, and assertion A forbids the two spellings
coexisting in one resolve dir, so a member's shell form vanishes the instant its
own port lands — while the live corpus of every criterion-4-binding sibling moves
with it. So **every binding member's live-tree comparison was captured against
one shared pre-descriptor snapshot, before the first descriptor of the batch
landed**: one capture for the batch, not one per member interleaved with the
landings. An incremental order silently compares each later member against a
corpus its predecessors already changed, and the resulting verdict reads like the
earlier ones without being the same claim. The fixture-pair runs carry no such
constraint — `gate-tests` is pruned from every live-tree walk — which is a second
reason the pair is the proof and the live tree is smoke.

**The capture's verdict, recorded in the register §The port-candidate criteria
sets for a gate-source auditor: no disagreement found on the pre-descriptor
tree, never parity proved.** Nineteen comparisons over the three binding
members — live tree with no argument, both fixture cases each, three edge roots
reached through `..`, three refusal paths, and two synthetic corpora built to
reach report arms no committed case can (a multi-offender scan and a
multi-alias corpus carrying a NUL-bearing file) — agreed on stdout, stderr and
exit code in every one.

**`check-docs-cname-parity` is the cut's finding, and it is the kind assertion C
cannot produce.** Its `couples=` is the single literal `docs/CNAME`, so the
derived substrate-sensitive set does not select it — and its *walk* nonetheless
reaches every tracked file, because its scan root defaults to the whole tracked
tree (`SITE_KIT_SCAN_ROOT`, whose value site-kit/SPEC.md owns) and the rule greps
each file's bytes for URL hosts. Every kit's `checks/*.sh` and
`*.gate` is inside the corpus it scans as content, which is criterion 4's
predicate verbatim, reached through the **walk** rather than through the trigger
field. It is the first worked instance in the direction where the couple clears
and the walk binds, and it is recorded beside the two of the opposite kind at
that criterion.

**`check-gate-fail-closed` is *not* excluded, and the reading that would exclude
it is named so it is not re-made.** §Meta-gate conservation for the binary
substrate dispositions it *Retired with cause* — but that table's subject is what
each meta-gate's assertion is worth **about a `.gate`-dispatched member**, never
whether the meta-gate itself may port. Its corpus is full in every vendoring
consumer, where shell is not an exception but the only substrate, so the rule
keeps its whole reason to exist. What is true is that its `check-*.sh` glob is
**not substrate-adaptive**, so a ported sibling leaves its corpus with no red —
which that same row already rules correct, a ported member having no shell to
lint.

**One hazard was probed and is false**, recorded because the mechanical reading
produces it: porting the last shell gate in a kit does **not** un-recognise that
kit as a kit root. The derivation's predicate is the *existence* of the `checks/`
or `smoke/` directory, not a `*.sh` glob, and the kit roots that already carry zero
`.sh` under `checks/` resolve normally.

**Assertion C was re-run fresh after the last descriptor landed, and the cut
adds no member** — on the rule §The first budget batch established, that a port
moves a declaration path and can move *other* members into or out of the derived
substrate-sensitive set, so the reading is never inherited. It was **measured
rather than argued**, by running the derivation over both trees: the same five
members are substrate-sensitive before the batch's first descriptor and after its
last, and each already carries a disposition. The derivation compares each
member's expanded `couples=` against the *resolved* declaration paths, which are
absolute, so the set is exactly the members whose couples carry an unanchored
`*.sh` or `*.gate` glob — a port changes such a member's extension and never its
membership. §Meta-gate conservation for the binary substrate gains no row.

**The cut found the transitive-couples rule diverging from the tree**, in the
direction that would have put a universal crate layer in every descriptor. The
rule now states its own scope, at §The non-gate arm where it lives; eight
descriptors were written against the tree's reading before it did.

**The seam was ruled per member rather than assumed, and the band made that
worth doing.** Every member reads consumer content — a message-pattern file, a
governed-path manifest, a suite roster, a docs root, a host and a scan root, an
exec-glob set — and a Rust `const` holding any of those would publish one
project's configuration as everyone's mechanism. None became a crate literal.
The discharge is criterion 6's strongest form rather than a concession: each
value crosses as a **bridged knob**, computed once in the owning kit's shell
library, so the binary holds no default to drift from. What the batch had to add
was the *spelling* — six values were defaulted inline where `declare -p` could
not see them, and a knob the bridge cannot find is its undeclared-knob refusal,
so each was resolved onto a name in its kit's library: `GATE_SDK_CORE_FILES_FILE`
and `EVIDENCE_KIT_RUNNER_DOC` under their own names, and `GATE_EXEC_GLOBS`,
`GATE_EXEC_PRUNE`, `GATE_MSG_PATTERN_FILES` and its `_LOCAL` sibling under
distinct ones, on §lib/gate.sh's rule that a whitespace scalar feeding an array
is the one case a resolved global earns a spelling of its own.

**One member's argument could not be ported at all, and that class is now stated
at the procedure** (§The non-gate arm): an argument selecting *where*
configuration comes from arrives after the bridge has already resolved it. It is
deleted rather than reimplemented.

**Delta 6's exclusion of `check-gate-exemption-tasks` was right twice over.** It
was excluded because a sibling rider widened its invariant in this same
iteration, and porting a gate in the iteration that changes its contract makes
the parity oracle chase a moving target. That contract then moved a *second*
time, when the same iteration scoped both its arms to the authoring tree
(§check-gate-exemption-tasks) — a repair this batch's own criterion-5 measurement
surfaced. A member whose contract moves twice in one iteration is the case the
exclusion exists for, and the batch would have paid for it had the exclusion been
read as bookkeeping.

### The fifth budget batch

**Seven members, a width the operator ruled fresh for this cut.** §The fourth
budget batch's 6–8 is that cut's own ruling and was not citable here: cut widths
are ruled per cut and never inherited, and a later selector reading the two
together as a standing envelope is reading a precedent that has now been refused
twice. Under the arm's record-only-findings rule this section carries the cut's
findings and no member roster — membership is derivable from the tree and the
count from `scripts/measured-claims.sh`'s `ported-gate-members`.

**The width was re-examined *after* the amendment was authored, on cost evidence
that had moved, and held.** The ruling was taken against a survey ranking the
corpus by declaration size and logic hazards; authoring found the cut's real cost
sits in interfaces instead, plus a pre-port library unit, two caller re-pointings
behind no gate, and one member whose largest work item that ranking cannot see.
The moved evidence was put and the ruling held, on grounds recorded rather than
left in a message thread: the member set was promoted unchanged; **drop-any-member
survives in the envelope**, so the executing stage keeps that relief without
anything being re-opened; and the added work is concentrated and named rather
than diffuse, two of its items being one-time. Recorded because a later reader
finding the member audit would otherwise reasonably conclude the width was never
re-examined against it.

**What is new is that the cut composed off a *filed, witnessed* cost survey
rather than off a property read at the cut.** §The fourth budget batch composed
by declaration size plus criterion cleanliness, reading both at its own cut; this
one took the seven cheapest of the fourteen takeable as ranked by a survey filed
under lifecycle-kit/SPEC.md §The survey record, whose witness the authoring stage
re-ran and passed. That makes the selection auditable after the fact and
re-runnable by a later reader, which a cut-time read is not. **The band is
exhausted downward, and that is the honest reading of the width**: the fourth cut
consumed the members at or under ~103 declaration lines and this one's ran 93 to
140, so the ceiling rose only because the floor did. Width seven bought what
width eight bought last cut and nothing more — session overhead spread over more
members, since cheapness is not a shared derivation.

**The cut's cost was in *interfaces*, not in logic, and that is its defining
finding.** The ranking it composed from is a ranking of logic. What it could not
see is that six of the seven members carry a command-line argument, two of them
documented on a governed surface — so the deleted-argument ruling had to be
adjudicated more often here than in the four prior batches combined. Applied per
member against the test §The non-gate arm now states, it bound **twice**, and
that section owns both the test and this cut's verdicts. The finding that
generalizes is the gap between the two: a cost ranking by size is silent about
interface removals, so a porting session prices them itself.

**Two live callers named a batch member's declaration path, and no gate would
have caught either.** This is §The third budget batch's lesson arriving twice in
one cut — a port's Point-5 reader enumeration covers gates, and a direct
shell-out from a non-gate script is outside every one of them — and both were
found by grepping the tree. `install-hooks`' identity rung *passed silently* on a
descriptor (§install-hooks), and this consumer's close-stage entry pre-flight
*refused the stage* (evidence-kit/SPEC.md §check-evidence-manifest, which owns
the consumer-side front-end that discharges it). Each landed **with its own
member's port commit** — a different sequencing from the pre-port library unit
below, and collapsing the three into one "pre-port fixes" bucket gets two of them
wrong. Neither is a library unit and neither is *reachable* until its own member
is being ported; what binds is that neither may land **after**, because the
window in which the descriptor exists and the caller has not moved is the window
the defect lives in. Same-commit makes it zero-width. The class is wider than the
two instances — three `gate-tests/*.test.sh` harnesses also invoked their subject
by literal path — so every member's port re-grepped the tree for its own
declaration path before the deletion landed, the enumeration being a grep and
never a gate.

**Four knobs the batch needed were bridge-blind, and the fix was a *pre-port
unit* rather than a per-member cost.** The bridge can carry only a knob whose
default is visible to `declare -p` after the kit library is sourced; a value
defaulted inline at its use site, or inside a helper's body, hits the emitter's
undeclared-knob refusal on the member's first post-port run. Three of the seven
members would have tripped it and no already-ported member declared any of the
four, so it landed **once, in front of the batch**, before the first descriptor —
an ordering constraint of the same standing as the shared snapshot below.
§The fourth budget batch paid this six times by finding it per member; recording
it as a precondition is what stopped the fifth paying it three more, and
§lib/gate.sh now states the precondition at the library's own contract.

**The cut executed as several build sessions, which turned the shared snapshot
from a moment into a *rev*.** The ordering constraint is §The fourth budget
batch's and applies unchanged — assertion A forbids both spellings in one resolve
dir, so a member's shell form vanishes the instant its own port lands while the
live corpus of every binding sibling moves with it. "Before the first descriptor
of the batch lands" is simply not reachable for a member ported in a later
session. It does not have to be: the pre-descriptor tree is a **named commit**,
the one landing the library unit above and nothing else, and a later member's
live-tree comparison is taken against a worktree of that rev. **Reap that
worktree off `git worktree list`, never off `git status`** — a scratch directory
is gitignored, so the status reads clean while an unreaped worktree still aborts
a clean-tree precondition, and it surfaces at a commit or a smoke rather than
where it was made. The verdict stays *no disagreement found on the pre-descriptor
tree*, never *parity proved*.

**One disagreement was found and adjudicated rather than hidden**, on the rule
that a disagreement is a finding against the rule and never a defect in whichever
side moved. `check-gate-tamper`'s shell form reported the added-exemption set
in an order that was not reproducible run to run once a commit added two matching
exemptions; the compiled form emits the same set byte-sorted.
Ruled as set semantics on the kit-roots cohort's precedent, and recorded at that
member's own section with the second behaviour change beside it.

**Fixture-pair pre-work was real on every member and was *most of the port* on
two.** Every one of the seven pairs was a single good case and a single bad case,
so any member with more than two branch outcomes was under-exercised by
construction, before anyone read a line. The finding that generalizes is §The
fourth budget batch's and holds here too — a clear criterion-4 verdict says the
parity oracle is not self-referential, never that the pair reaches every arm —
and this cut sharpens it into a second class: two members were not "widen the
pair" but **"write the pair"**. One exercised one of the eight violation classes
its rule carries; the other ran its fixture arm in *both* cases, so the function
reading gate-file bytes through git had no committed case at all and one
assertion's violation had none in either direction. That second one is the shape
to look for — a pair that injects the corpus cannot reach the code that derives
it, however many cases it has — and the discharge is a `gate-tests/*.test.sh`
sibling driving the live arm, not another injected case.

**Two takeable members were excluded by name, and both grounds were sizing.**
`check-template-copy-parity` is clean on criteria 4 and 7 and is the first member
a later cut should reach for; its cost is concentrated where the line count hides
it, in reproducing a hand-rolled `case`/`esac` surface classifier faithfully.
`check-graph` is excluded on a stronger ground and it is a ruling rather than a
deferral: it emits a self-contained HTML artifact, which makes it a non-gate arm
that must be **designed before it is ported** (§The non-gate arm). Bundling it
into a budget batch would violate the never-as-one-cohort property the mechanism
rests on — a batch whose members must be droppable cannot carry a member whose
design is a prerequisite for the rest of its own work. It wants its own
iteration.

**Criterion 5's residual was measured against the post-batch registry and
corrected the growth predicate rather than the batch** — the amendment's
at-most-five prediction held as a bound and missed as an estimate, because the
residual is measured over one profile's kit set and two of the batch's seeded
members ship in kits that profile does not carry. §The port-candidate criteria,
criterion 5 owns the number and the corrected predicate.

**Assertion C was re-run fresh after the last descriptor landed, and the cut adds
no row** — on the rule §The first budget batch established, that a port moves a
declaration path and can move *other* members into or out of the derived
substrate-sensitive set, so the reading is never inherited. It was **measured
rather than argued**, by running the derivation over both trees: the same five
members are substrate-sensitive before the batch's first descriptor and after its
last. One row was already written ahead of its own trigger and must not be
re-derived as new — `check-gate-binary-fresh` is a batch member *and* carries a
disposition recording that it reads declaration paths as a set, so its port is
the case that switches it on rather than its blind spot. And this is gate-sdk's
meta-gate conservation assertion C, **not** lifecycle-kit's `check-stage-entry`
assertion C: the two share a letter and nothing else, and the second is the one
this cut's own cross-component amendment tripped at the next stage's entry.

**The seam was ruled per member rather than assumed.** Every member reads
consumer content — a gates roster and fixture roots, a workflow evidence manifest
and a queue, a governed queue plus the amendment glob, an identity manifest, a
meta-path exemption set — and none became a crate literal. The discharge is
criterion 6's strongest form rather than a concession: each value crosses as a
bridged knob computed once in the owning kit's shell library, so the binary holds
no default to drift from. The one place a *kit* literal was admitted is a
wire-format version string, which is kit mechanism rather than consumer
vocabulary, and it is held to the shell library by a unit test that executes it
(evidence-kit/SPEC.md §check-evidence-manifest).

**The cut is where §The fourth budget batch's kit-root probe became
load-bearing.** That batch probed, rather than argued, that porting the last
shell gate in a kit does not un-recognise the kit as a kit root — the predicate
is the *existence* of a `checks/` or `smoke/` directory, not a `*.sh` glob. This
cut is the case that would have believed the mechanical reading: porting one
member left delegation-kit with **no** `check-*.sh` at all. The directory and its
two sibling descriptors remain, so the root does, and `check-kit-enum` — which
reds on a *set inequality* in both directions and is therefore not clearable by
inspection — stayed green.

**This batch carried that rule into kits with no ported member of their own kind
of content**, contributing one member each to canon-kit, evidence-kit and
delegation-kit. The ranking survey's claim that one of them was the *first port
into evidence-kit* was **false** and is corrected here rather than carried — that
kit already dispatched two members to the binary. What was true, and what cost,
is narrower: the member's own helpers had no Rust counterpart, which is a
helper-level fact and not a kit-level one.

### The sixth budget batch

**Six members, a width the operator ruled fresh for this cut** on 2026-08-19:
the whole takeable tier bar `check-graph`, ruled with the raised declaration band
in hand and against it. Cut widths are ruled per cut and never inherited, and the
five sections above are each their own ruling rather than a standing envelope. As
with every batch section, this one records the cut's **findings and no member
roster** — membership is derivable from the tree and the count from
`scripts/measured-claims.sh`'s `ported-gate-members`. The fifth cut's
drop-any-member relief rides in this envelope by the same ruling, available for
cause and never as a budget lever.

**Taken whole, the cut leaves `check-graph` the one takeable member the oracle
still counts**, and that member is ruled out of every budget batch rather than
out of this one (§The fifth budget batch). So the takeable *tier* is exhausted
by the six, and the budget arm has no further cut to compose until a
`# port-until:` hold releases. The composition ground was re-read here rather
than inherited: `port-blockers.sh --group` reported seven groups of exactly one,
0 undecidable, so the size arm stays permanently exhausted and this is a budget
cut rather than a cohort.

**The band moved *up*, and the width is the honest consequence.** The fifth cut
ran 93 to 140 declaration lines; this one runs 127 to 240. Six members here are
heavier than seven were there, and the reason is the one that exhausted the band
downward: the floor rose because the cheap members are gone. Width six buys what
width seven bought last cut and nothing more.

**Criterion 4 bound on four of six, and two of the four are new rows for the
register** (§The port-candidate criteria, criterion 4). `check-gate-exemption-tasks`
binds in **every** configuration — in an authoring tree its declaration is in the
scanned set and in a vendored consumer it is in the out-of-scope set the skip count
still reads — which is the register's first member with no clearing configuration.
`check-knob-default-coupling` is a **third** couple-clears-walk-binds instance and
the sharpest yet: its trigger field is one level shallower than the walk beneath
it, so assertion C's derived set structurally cannot report it while the walk binds.
`check-spec-embedded-source`'s verdict is taken **conservatively without ruling the
class** its own entry owns, on the ground that a conservative verdict costs a
fixture widening and cannot be wrong in the harmful direction. The two that clear
are properties of a gate against a consumer's config rather than of the gate.

**Criterion 5's price, measured on both sides.** The binary-less leg reports
**twenty** omitted members against the post-batch registry, where the same leg on
the pre-descriptor rev reports **seventeen** — a growth of **exactly three**, which
is the prediction rather than a bound: three of the six are `zero-config`, and this
is the first cut where intersecting them with the measured profile's kit set is a
**no-op**, the profile carrying every kit the members live in. Both
numbers were measured, from a clean checkout of each rev, rather than one measured
and one reasoned.

**The judgment is *accept and declare*, ruled here against this subtraction rather
than inherited.** What the batch removes is larger than any prior cut's: §The
canonical-spec `spec_canonical_specs` cohort accepted its own residual **because**
the canonical-spec corpus still carried a shell auditor, and this batch ports that
auditor. On a binary-less host the corpus is now unguarded at install — a class
emptied, which is exactly the aggregate cost this criterion forbids landing
unpriced. It lands anyway, on the terms that criterion states: the loss is
*declared* in the consumer's own registry rather than arriving as a broken battery,
the 2026-08-09 directive ports the whole corpus, and the subtraction shrinks as
targets are published rather than being repaired by the cut that caused it. That
cohort's grounding sentence is corrected where it lives, since nothing gates it and
it would otherwise stand as a false claim.

**Assertion C was re-run fresh after the last descriptor landed and yields no new
row.** Five of the six members already carried a disposition in §Meta-gate
conservation for the binary substrate, each written about *other* members being
`.gate`-dispatched; after this cut every one of those rows describes a ported
member and each takes the sentence the table already uses for that transition. The
sixth, `check-prose-tells`, is named nowhere and needs none: its couples reach no
registry member's declaration path, so the derivation does not select it.

**The bridge precondition is discharged rather than inherited, and what replaced it
is subtler.** §The fifth budget batch landed a pre-port library unit because four
knobs its members needed were bridge-blind. Twenty-eight knob names cross here and
**not one** is bridge-blind, so this cut owes no library unit — recorded so the next
selector does not read that precondition as a standing cost. The live trap is now
declaring the wrong *spelling* of a knob that has a default: four names this batch
touches are defaulted only inline and are subsumed by a resolved sibling the bridge
already carries, so declaring any of them by name hits the undeclared-knob refusal
on the member's first post-port run. The verdict is taken at the library and never
at the use site, because neither direction of the trap is visible from there.

**Ordering bound *within* the batch, which is new.** One member's corpus contains
another member's declaration as content, so porting the second moves the first's
clean-line counts. Every later member's comparison was therefore taken against a
worktree of a named pre-descriptor rev — assertion A forbids both spellings in one
resolve dir, so a member's shell form vanishes the instant its own port lands and a
cut executing across several sessions cannot reach "before the first descriptor".
The verdict each run supports is *no disagreement found on the pre-descriptor
tree*, never *parity proved*. `check-reads-couples` was ported **last**, on the
ground that while its shell form existed it was the batch's only cross-substrate
oracle: the one second implementation available to audit the five siblings' new
descriptors.

**Fixture pre-work was most of the port, and the class it adds is that an arm
reachable only through a `.gate` member has no case anywhere.** In the live tree
the overwhelming majority of registry members are descriptors, so the arm doing
most of the work was the arm no case reached — and no fixture anywhere shipped a
descriptor before this cut. Two members carried that hole and both are closed;
three valves that no committed case exercised are now exercised; and one member's
root default is a derivation no injected case can reach, discharged by a sibling
harness driving the live arm rather than by a third case.

**Three defects the port reproduces rather than repairs**, on the owning spec's own
rule that a refusal the shell form never made is a verdict change across the seam —
which is precisely what the parity run holds invariant, and precisely what the
natural port instinct is to add. Each is filed to the gap inbox as its own unit.
The exception is a governed sentence that had drifted from the code it describes:
there the port takes the code and **corrects the sentence**, because that is the one
case where reproducing leaves nothing false.

**The seam was ruled per member, and one member's is a *privacy* boundary rather
than a config one.** Every member reads consumer content and none became a crate
literal; the discharge is criterion 6's strongest form, each value crossing as a
bridged knob computed once in the owning kit's shell library. Two inlining risks
were named because each reads like a natural constant: a wire-kind string, and a
pair of merged consumer-extended vocabularies the library unions **before** the
bridge reads them, so declaring the extension names beside them would double every
consumer token while declaring only them would drop the bundled set. The privacy
one is `check-template-copy-parity`'s pattern discard, which is what keeps a
consumer's rule vocabulary out of the crate and which no committed case could
catch a port for losing.

**One design fork was escalated at authoring and ruled at build**: a compiled
`check-reads-couples` must resolve a knob whose name is not known until run time.
The ruling is a **derived union** computed off the registry and carried across the
existing bridge as data, with the per-kit prefix family as the fallback; the two
citations that read as a fork answer different questions and are not opposed, and
spawning an interpreter is refused as converting a transitional dependency into a
permanent one. §check-reads-couples owns the ruling and its grounds.

**The cut's cost is in *fresh derivation*, not in interfaces — the exact inverse
of the fifth, and the pair is the finding.** That batch's defining result was
that six of seven members carried a command-line argument and the
deleted-argument ruling bound more often than in the four prior batches combined.
The same probe here answers the other way: five of six carry an argument, six
arguments in all, and every one ports unchanged — not one deletion. Where the
cost actually sits is invisible to a line count in both directions: embedded awk
with no Rust counterpart, a hand-rolled surface classifier whose value is in its
incidental behaviour, a fence tokenizer with a restricted info-string grammar.
**A cost ranking by declaration size is silent in both directions**, blind to
interface removals there and to derivation depth here, so a porting session
prices both itself (§The non-gate arm; §The first cohort, and the rule that
selects the next states the same limit at `--group`'s `lines=` column, and this
batch is the second of the two consecutive cuts it cites).

### The canon-kit `spec_manifest_files` cohort

**The shared derivation is compiled.** `native/src/spec.rs` carries all three
branches of `spec_manifest_files`, the kit-root path prune, the manifest-prose
walk driver and the count grammar, so the derivation was ported once and is
proved by every member that calls it — the same economy the first cohort's
pairing bought, at seven members instead of two.

**Parity was bought against the branches this repo does not run, and that is the
transferable half.** This tree sets `CANON_KIT_MANIFEST_FILES`, and sets
`CANON_KIT_SCAN_KIT_ROOTS` to the value that disables pruning
(canon-kit/SPEC.md §Layout and configuration owns both), so every live
invocation takes the explicit-glob branch and the kit-root prune is a no-op here. A live-tree comparison therefore
proves nothing about the default walk or the prune — and *the live tree is the
oracle a session trusts most*. The port stood up a differential edge tree with
both knobs at their defaults, carrying each of `SPEC.md`, `README.md` and
`CLAUDE.md` twice — once under `templates/`, once at a vendored kit root — plus
the ancestor-root case and a kit root that is not a descendant of the scan root.
It caught a prune that never ran: a scan root of `.` was being absolutised to a
`.` component appended to the cwd, so every path-prefix test failed silently.
**No fixture pair and no live-tree run could have surfaced that**, which is the
lesson `check-roadmap-fresh` taught one cohort earlier arriving through the
configuration rather than through a fixture.

One difference the port takes deliberately: the default walk's `find(1)` order
is a filesystem artifact, and the Rust walk sorts. Every member reaching its
corpus through `sort -u` stays byte-identical; the one that prints its corpus
unsorted is set-identical with a different line order, asserted as such rather
than normalised away.

**Seven of ten at the cohort's own cut, with three members sequenced behind an
ERE engine — the same sequencing, found the same way.** The cohort was ruled at
ten on a design that removed the `bash` spawn from the three members reading a
consumer-supplied *command*. That design reached only half the problem, and the
other half was found at implementation:

- **`check-install-claim`, `check-payload-claim` and `check-manifest-temporal` —
  an ERE engine.** Each **interprets** a consumer-configured POSIX ERE rather
  than transporting one: the first against `CANON_KIT_INSTALL_SECTION_RE` and one
  pattern per transport, the second one pattern per disclosure class, the third
  every `CANON_KIT_TEMPORAL_MARKERS` entry. Bridging the vocabulary as *data*
  removes the interpreter the binary would have spawned; it does not remove the
  regex the binary must then apply, so a port owes an engine. Sizing one to this
  consumer's patterns is **foreclosed** by the argument this section already
  makes for `check-queue-prose-precondition` and criterion 6 makes for globs;
  that foreclosure binds the pattern **grammar** the engine accepts and not its
  **API**, and §The POSIX ERE matcher owns the distinction. **Operator-ruled
  2026-08-12 at build.** The engine is paid and those three members ship
  compiled: §The POSIX ERE matcher is their contract, and this cohort's ten are
  all ported.

  `check-manifest-temporal` is worth naming separately, because every mechanical
  screen puts it *in*: it clears criterion 4, spawns nothing, and reads its
  corpus through the very derivation this cohort compiled. What stops it is that
  its marker set is consumer config the rule applies as a pattern — and the
  **shipped kit default** already carries an alternation, so the engine is
  load-bearing for a consumer who overrides nothing at all. A cohort sized on
  "clears criterion 4" would have taken it and discovered the engine mid-port,
  which is the failure mode criterion 7 exists to prevent.

  **It is a class with two members, not a one-off**, which is why the shape is
  named here rather than the gate. `check-spec-derivable-section` sits in the
  identical position: `CANON_KIT_DERIVABLE_POINTER_REGEX`'s shipped default
  (canon-kit/SPEC.md §lib/spec.sh) is an alternation too, so the same
  override-nothing consumer makes the same engine load-bearing. Both are on the
  nine-member roster `cohort-held-members-port-prerequisites` carries, so a
  selector reading that roster is already warned; what a second instance adds is
  the screen to run before trusting a mechanical one — **read the kit default,
  not only the knob**, because a knob documented as consumer config is exactly
  where a mechanical screen stops looking.

These holds are sequencing rather than exclusion under the rule §The
port-candidate criteria fixes and this section restates for the queue-kit pair
above; what each remaining member owes is filed on
`cohort-held-members-port-prerequisites`. The roster and its kits live on that
entry and are not restated here, because a count copied into prose is the drift
this correction is itself repairing.

### The kit-roots `gate_kit_roots` cohort

**Five members, and the axis is the corpus derivation rather than the kit:**
`check-kit-registration`, `check-smoke-entry-guard`, `check-test-hermetic`,
`check-assertion-strength`, `check-template-registry-parity`. All five derive
their corpus from `gate_kit_roots` and nothing else structural — a sweep of the
kit roots, then a fixed literal sub-path under each (`smoke/install.sh`,
`smoke/violation.sh`, `gate-tests/*.test.sh`, `smoke/*.sh`, `bin/*.sh`,
`templates/*.list`). That derivation was **already compiled**: `gate_kit_roots`
and `gate_kit_roots_rel` cross the bridge as resolved values, landed with the
canon-kit cohort, so the axis that made the first cohort cheap was paid for before this one
started. What each member adds on top is a per-file text test, which is the whole
of its rule — the first cohort's economy at five members instead of two.

**The fifth member ports under criterion 4's own predicate, and the ground is
the second over-selection path rather than the first.** Assertion C's runtime
derivation reported `check-template-registry-parity` substrate-sensitive: its
`couples=` carries `kit:*/*.sh`, which expands to `<root>/*/*.sh` once per kit
root and covered the `.sh` gate declarations then under any kit's `checks/`
— `gate-sdk/checks/` among them until `shell-gate-tail-port` emptied it of that
spelling, and `site-kit/checks/` last, which emptied the set outright and with it
the ground on which the derivation selected this member (criterion 4's worked
instance owns what that costs the paragraph there). The port verdict below stands
on its own reasoning and is unaffected. It is **not** a
reverse-trigger couple —
the gate really does read `*.sh` names as content, through
`git ls-files -- '*.sh'`. It is a **content couple wider than the walk**: the
names it reads are only those under a `<kit>/<name>/` directory that a sibling
`<kit>/templates/<name>.list` registers, and no kit ships a template registering
`checks/`. Against this tree that walk reaches one live registry, drift-kit's `kpis.list`,
held in population by native dispatch since the 2026-08-29 cut rather than by a
sibling directory, with `gate-sdk/templates/msg-patterns.list` skipped for want of
either. The
port therefore changes nothing the gate reads and the parity proof is not
self-referential. Recorded at this length because every mechanical screen puts
this member *out*, and because the amendment that proposed the cohort argued it
on the reverse-trigger clause — a ground that does not cover it, corrected here
against the predicate §The port-candidate criteria actually states.

**One library move was bought before the first gate, and it is the class
criterion 7's worked example exists to surface.** `check-kit-registration`
resolved `GATE_SDK_REGISTRY_DOC` and `GATE_SDK_RUNNER_DOC` inline, so
`lib/gate.sh` defined neither and the config bridge's does-not-define refusal
would have fired on every invocation of the compiled member. Both defaults moved
into the library, under the **consumer knob names** rather than renamed the way
`GATE_PRUNE_DIRS` is: that rename buys a distinct spelling because a whitespace
scalar feeds an array, and a scalar-in/scalar-out knob has no such collision to
avoid (§lib/gate.sh). No consumer configuration changed.

**Three deliberate differences, asserted rather than normalised away**, in the
shape the canon-kit cohort's sort-order note set:

- `check-assertion-strength`'s token→code map came out of `awk` in hash order and
  comes out of the crate sorted. Set-identical, ordering different, and it reaches
  output only through a multi-finding message.
- `check-template-registry-parity`'s two `comm` arms are computed by set
  membership. `sort`'s collation is locale-dependent and a set difference is not,
  so the port implements the rule the contract states rather than the mechanism
  the shell reached for.
- A member taking a positional root resolves its **relative** kit roots against
  that root, exactly as the shell did — but the bridge spells *every* root
  relative, so where the shell would have had an absolute root and ignored the
  argument, the port honours it. The two agree whenever the positional root is
  the invoking directory, which is every documented use: the argument exists to
  steer a fixture tree whose `gate-sdk-config.sh` names relative kit roots.

**Parity was proved on 34 cases while both implementations existed** — each
member's `good/` and `bad/` case, the live tree, every arm each member carries,
and a constructed differential tree for the branches neither reaches:
`check-kit-registration`'s assertion B alone and both assertions together (the
blank line between the two finding blocks fires only there),
`check-test-hermetic`'s credential-leak assertion and its exempt-marker escape,
`check-assertion-strength`'s window cut short by a following invocation and its
own exempt marker, and every fail-closed refusal. Byte-identical, exit codes
included, on all of them.

**Two readers of the narrowed corpus were missed by the amendment's enumeration
and found by the battery**, which is worth recording because the enumeration was
the delta that claimed to be exhaustive:

- `check-gate-assertions` resolved a gate name to its declaration and looked for
  `# assertion` markers there. A `.gate` descriptor carries none and cannot — the
  field roster is closed — so three members with enumerating contracts read as a
  retrofit obligation. Repaired by the resolution §check-gate-output already owns:
  a `.gate` member's markers are looked for in its implementation module, and the
  marker grammar accepts either comment leader, because the leader is the
  substrate's and the marker is a code marker either way. **The repair was itself
  incomplete in this repo and only the aggregate-price oracle showed it**: a
  vendored consumer receives the descriptor and never the crate, so the module
  path resolved to nothing and every adopter's battery reded. Those members are
  now skipped-and-counted with no crate present. Recorded because the sequence is
  the lesson — a tree-green repair is not a consumer-green one, and criterion 5's
  measurement is what distinguishes them.
- `check-docs-kit-parity` is a consumer gate that **wraps** `check-kit-registration`
  by executing its path. It now reaches it through `gate_command`, the argv being
  the one spelling that survives either substrate. Recorded because a wrapper is
  a consumer of a ported member that no kit-side roster names.

**The aggregate price is measured, not reasoned, and the reasoning is written
first so the measurement has something to falsify.** All five members assert over
**kit-authored** files — a vendored kit's own `smoke/`, `gate-tests/`,
`templates/` and registry rows — never over adopter-authored content, so a
binary-less consumer losing the five should lose no class of adopter-authored
content at all: the omitted roster grows by five and the residual behind it is
unchanged.

**The run agrees, and the price is zero.** Against the post-cohort registry every
profile's battery is green on a freshly vendored consumer, with the ported
members omitted-and-declared exactly where the payload carries no binary — the
arm that asserts that equality is the suite's own, so no roster is transcribed
here, and that assertion is the residual's oracle rather than the value arm the
criterion named at the time. The value arm, a claim about the product and not
about the residual, reports the same
verdict on every profile it reported before: the planted defect is green, and
green once fixed. The suite's only failure is the markdown-link class the held
row already owns, so no class this cohort could have emptied changed hands. The
falsifier was available and did not fire: a newly lost class would have shown as
a value-arm class this consumer used to catch, or as a red battery, and neither
appeared. The `installer_smoke fail` row that stood held in
`.workflow/validate-baseline.txt` at the time was **not** this cohort's price: it
recorded the markdown-link hole `port-criterion-aggregate-cost-blindness` half
(2) owns, which this cohort neither widened nor repaired. That row was earned out
to `pass` in `97683db2`; the sentence is kept in the past tense rather than
deleted because it is what discharged this cohort's price question.

### The canonical-spec `spec_canonical_specs` cohort

**The eighth cohort, and the first selected by running the selection rule rather
than by reading candidates off the tree.** Two members —
`check-spec-dod-singleton` and `check-spec-derivable-section` — sharing the group
key `libs=fail_closed,spec_canonical_specs globs=-`, taken from a
`bash gate-sdk/bin/port-blockers.sh --group` run that scanned **104** members,
formed **48** groups, reported **0 undecidable** and excluded **40** already
ported. The undecidable count is recorded because it is the bound on the claim
that this group was the largest: at zero, the partition speaks for the whole
still-shell registry and the ordering rule was applied against complete
information. That is the first cohort able to say so, and it is a property of the
**repaired** scan rather than of this cohort — the tokenizer truncation that made
every earlier cohort's roster a silent under-report is what the zero rests on
(§port-blockers).

**Two larger groups were rejected, and recording why is what keeps the rule
usable.** The run's largest group carried 14 members on the key
`libs=fail_closed globs=-`. It is **not a cohort**: `fail_closed` is the
fail-closed guard and derives no corpus, so those members share the *absence* of a
derivation, which this section already rules is not a shared derivation, and
reaching that state through a non-empty key does not change the substance
(operator-ruled 2026-08-14). The tool's own contract predicts this reading — its
key is deliberately advisory and a group whose members' `couples=` visibly diverge
is a finding the selecting session adjudicates, never a cohort the tool cut. The
next group by size, `check-install-disposition` + `check-readme-roster`, clears
every mechanically-derived column and is still **not takeable whole**:
§Meta-gate conservation for the binary substrate rules `check-install-disposition`
a gate that **stays shell**, because its assertion is about the declaration and
dispatch relation and a compiled form could pass itself with a broken binary. It
is a member the port does not take, on the same footing as `check-crate-arms`, and
a selector reading only the tool's columns would have taken it. The remaining
two-member groups fail a column outright: one carries a `ruby` requirement
(criterion 7), the other is this cohort.

**What the cohort fails is criterion 3, and that is why it outranks the
alternatives.** Both members are `tier=align-only`, so they do not land in the
generated hook and a green `check-graph` after the port is not the end-to-end
proof of the manifest surviving the substrate change that criterion 3 buys. The
manifests are still read — the graph artifact covers every registered member —
so what is lost is the hook arm of that proof and nothing else. Against it, every
alternative of equal size fails **criterion 4 or 7**, which this section sequences
**last and budgets for** because each carries a design problem rather than a
weaker proof. A group failing only criterion 3 is therefore ahead of a group
failing 4, and it is also the larger of the two once the held member leaves group
2. `scripts/`-declared members were excluded from this cohort by its amendment;
the run surfaced none at or near the top, so the exclusion never bound.

**Criterion 6 is discharged by the corpus derivation already being compiled.**
`spec_canonical_specs` is the SPEC-name find, `templates/`-filtered and kit-root
pruned — every piece of which `native/src/spec.rs` already carried inside
`manifest_files`' default branch since the canon-kit cohort. The port **lifts it
out** rather than writing a second copy beside it, so the manifest set and the
canonical-spec set cannot disagree about which specs exist. The shell form stays,
and **the reason it stays has moved twice while the verdict has not**. It was
originally the *unless* clause's live-consumer disposition, the same one
queue-kit's `lib/queue.sh` takes: the caller set was two when this was written,
one after §The sixth budget batch ported one of them, and **zero since
`shell-gate-tail-port` ported `check-surface-duplication`** — `spec_manifest_files`
now has no caller outside `canon-kit/lib/spec.sh` itself, and neither does
`spec_canonical_specs` beyond that library's own default branch. An empty caller
set is the *dead twin* case, whose disposition is deletion rather than a standing
parity obligation — but that disposition carries an explicit bound, **undocumented
surface**, and these two are documented helpers named by canon-kit/SPEC.md
§lib/spec.sh as the manifest set the narration gate family shares. So the ground
is the bound rather than a live caller, which is the distinction a later reader
must not collapse: a documented shell helper survives its last caller, and the
deletion disposition reaches only the helpers no section names.

**The edge-root arm disagreed, and the compiled side read the rule correctly.**
Parity ran over both fixture pairs, the live tree at four scan-root spellings and
a differential edge tree at nine knob settings — 30 comparisons, byte-identical on
stdout, stderr and exit code, run while both implementations still existed. Two of
those comparisons failed first: at a `..` scan root the shell prune kept **22**
canonical specs where the compiled form kept 11. `_spec_prune_kit_roots` is a
prefix test, and it normalised neither the scan root nor the file paths, so a
`..` component made every comparison fail and pruned nothing at all — the corpus
silently widening rather than reddening. The rule it is measured against says a
file under a kit root that is a strict descendant of the scan root is excluded,
and at a `..` root the kit roots are such descendants; the compiled side is
therefore right and **the shell was repaired before the port**, so the parity run
proves the repaired behaviour rather than freezing the defect. This is the same
class §The port-candidate criteria records the comment cohort finding, reached
through the scan root instead of the file path, and it is the standing evidence
for that criterion's rule that a disagreement is adjudicated against the
requirement and never against whichever side moved.

**What the repair's oracle can and cannot see, stated because the cohort paid to
find out.** `check-spec-dod-singleton.test.sh` gains a `..`-root case. It was
verified to red under an **asymmetric** break of the normalisation and to stay
green under a **symmetric** one — dropping `..` from every path alike leaves the
prefix test succeeding by coincidence, exactly as an end-of-file balance check
cannot see an early tokenizer pop. An oracle for a normalisation defect must
therefore break one side only, and a control that transforms both proves nothing.
The case holds the **compiled** prune, since the member now dispatches to the
binary; the shell twin the surviving callers use is covered by nothing, which is
filed rather than absorbed (`spec-prune-normalisation-shell-oracle`).

**Criterion 5's price, measured.** The binary-less leg reports **twelve** omitted
members against the post-cohort registry, where the comment cohort recorded ten.
The growth is **two against two ported members** — both are `zero-config`, so both
are seeded into a freshly initialised consumer's registry and both are therefore
losable, which is the growing direction of the same install-disposition predicate
that made the lifecycle-kit cohort's growth zero. Measured after this cohort's own
commit, from a clean tree, through the repaired invocation contract: the smoke
resolves its own tree from its script path and passes it to the packer as
`--root`. The first attempt refused instead of measuring, because a concurrent
edit dirtied the worktree mid-run — the cleanliness test is per pack invocation
and names the root it read, so that refusal is the instrument working rather than
a flake.

**The judgment is *accept and declare*, and what distinguishes it from the comment
cohort's identical verdict is that no class empties.** On an uncovered platform an
adopter loses the Definition-of-Done singleton and the derivable-section density
budget, declared in its own `gates.list` rather than arriving as a broken battery.
When this was written the canonical-spec corpus still carried a **shell** auditor
there, `check-spec-embedded-source`, so what an uncovered host lost was two
structural assertions over a corpus that remained guarded. **§The sixth budget
batch ported that auditor, and the sentence is corrected here rather than left
standing as a false claim** — nothing gates it, so it would have outlived its
subject silently. What now survives that corpus on a binary-less host is one gate
no `init` seeds and that sits in no generated hook, so the corpus is **unguarded
at install**: a class emptied, which is the aggregate cost criterion 5 forbids
landing unpriced. That batch's own section carries the measurement and re-rules
the judgment against its own subtraction rather than inheriting this one, because
the subtraction is larger than any the prior cuts made. The two rivals are refused with cause and the first
is refused harder here than usual: restoring the class shell-side would reinstate
a second `spec_canonical_specs` implementation, which is the exact duplication
this cohort's criterion-6 discharge exists by removing, and enforcement-first
ranks removal above gating it; a binary-gated declaration the adopter receives is
what the omit path already is. The honest limit rides with the ruling — this is a
real subtraction for an uncovered host, it lands because the 2026-08-09 directive
ports the whole corpus, and it shrinks as targets are published rather than being
repaired by the cohort that caused it.

### The POSIX ERE matcher

**Three members, and the engine is what the cohort buys.**
`check-install-claim`, `check-payload-claim` and `check-manifest-temporal` — the
members §The canon-kit `spec_manifest_files` cohort held on an ERE engine — port
together, operator-ruled 2026-08-13. A five-member clean-group alternative and an
eight-member superset were both offered and declined, so the cohort does not
widen. Three is the smallest cohort since the first, and on member count alone
the selection rule above picks elsewhere. What it buys is the engine: a POSIX ERE
matcher is owed by **nine** members across the kits
`cohort-held-members-port-prerequisites` rosters, and it is the largest single
piece of work the port has named. Paying it against three members retires the
blocker for all nine, which is the second worked exception to the largest-set
selection rule, beside `check-roadmap-fresh`'s.

**Criterion 4 binds on none of the three, and the verdict was taken by running
the derivation at the cohort cut.** Assertion C selects `check-install-claim` and
`check-payload-claim` as substrate-sensitive through their `couples=` glob
`scripts/*.sh`, which covers a gate declaration path. Both selections are the
**reverse-trigger** over-selection criterion 4 already names: that couple exists
so an edit to this repo's transport and disclosure emitters re-runs the gate that
consumes them, and neither gate reads a declaration path as content. What all
three scan is `spec_manifest_files`, the governed markdown set, inside which no
gate declaration lies. Both sensitive members carry a conservation row (§Meta-gate
conservation for the binary substrate); `check-manifest-temporal` earns none.

**The owed engine is a matcher, and that correction is the sizing.** Sizing the
engine as *"an ERE engine plus `gsub` semantics"* puts a substitution engine
inside the owed work, and it is wrong in the direction that costs the most.
**Eight of the nine members apply their consumer pattern only as a match test**;
`check-deprecation-task` alone extracts a span from one, and every `gsub`/`sub`
in the whole set runs over a pattern baked literally into awk source, which a
port hand-compiles. That is the same ground that correctly screened
`check-comment-tier` out of the roster. The owed engine is therefore **a POSIX
ERE matcher with leftmost-longest span reporting, and no substitution engine or
capture-group replacement**.

**"Sizing is foreclosed" binds the pattern language, not the API surface.** The
foreclosure §The canon-kit `spec_manifest_files` cohort states, on criterion 6's
globs argument, binds the **grammar** the engine accepts: the config surface
permits what this consumer happens not to write, and a narrow reader silently
mis-scans the first consumer who writes one. What the sizing correction above
touches is the **API** — what the members *do* with a pattern once it matches —
which is fixed by their own source rather than by what a consumer may write, so
no future consumer can turn a match test into a substitution. Reading the
foreclosure across both axes would buy a `gsub` implementation with no caller in
the corpus that justifies the engine.

**Why the engine stays hand-written now that the crate may take dependencies.**
This section once justified the hand-roll partly on the crate vendoring nothing;
that clause is retired — the settings cohort took `serde_json` under a stated bar
(§The settings cohort, and the crate's first dependency), so *"why is there a
hand-written regex engine?"* is a live question rather than a settled one. The
answer does not rest on the retired premise: the contract below is **POSIX
leftmost-longest span reporting**, which is the semantics `awk`'s
`RSTART`/`RLENGTH` gives and which the ecosystem's ordinary matchers do not —
leftmost-**first** is the common default, and this section already records that
the two agree on every `is_match` and disagree on spans. A dependency with the
wrong span semantics would be a silent regression in exactly the place a
boolean-only oracle cannot see. Replacing the engine is therefore a costed design
question with a real candidate set, not a cleanup a passing cohort performs.

**The contract.** `native/src/ere.rs` accepts **POSIX ERE in full**: alternation,
concatenation, `*` `+` `?`, intervals `{n}` `{n,}` `{n,m}`, grouping, `.`,
anchors `^` `$`, bracket expressions with ranges, negation and the POSIX
character classes, and backslash escaping of every special. Its public surface is
three items and no more:

- `Ere::compile(pattern) -> Result<Ere, EreError>`
- `Ere::is_match(&self, hay: &str) -> bool` — awk's `$0 ~ p`
- `Ere::find(&self, hay: &str) -> Option<(usize, usize)>` — the
  **leftmost-longest** span as byte offsets, the `RSTART`/`RLENGTH` pair awk
  reports

There is no `replace`, no `replace_all`, and no capture-group accessor; adding
one is a design decision with its own reader rather than an omission to fill in.

**The promotion trigger for a fourth item, recorded rather than left to be
re-argued.** `check-queue-prose-precondition` ported with **one** reader for
substitution, and took it as a private loop over `find` inside its own module
rather than as a public engine API — `find` reports leftmost-longest, which *is*
awk's `gsub` match rule, so a caller-side loop reproduces `gsub` with no engine
surface (`replace_all` in `native/src/gates/queue_prose_precondition.rs`, held by
the differential oracle below). One rule of that loop is owned here rather than
there, and its `spec:` line cites back: **an empty match advances one
character** — stated because its failure is a silent infinite loop rather than a
wrong answer. A **second** ported member needing substitution is what
promotes that loop into `ere.rs` as its fourth item, with the differential oracle
below widened to `awk 'gsub(p,r){…}'`. Until then a private loop with one caller
is cheaper than a public contract with one caller.
The engine is **byte-wise**, the C-locale semantics the span arithmetic of its
callers assumes: `find`'s offsets are handed to byte-indexed slicing, and a
char-wise engine shifts every one of them on a multi-byte glyph. Interval bounds
are capped at the POSIX `RE_DUP_MAX`, and the compiled program at a fixed
instruction count, because an interval expands by copying its operand.

**Leftmost-longest is the semantics, not an implementation detail.** POSIX ERE
alternation is leftmost-**longest** and a backtracking matcher gives
leftmost-**first**. The two agree on every `is_match` answer and disagree on
spans, so a boolean-only engine can be built on the wrong semantics and never
show it, until `check-deprecation-task` ports and extracts the wrong marker from
`(deprecated|deprecated-since)`.

**An unsupported construct is a fail-closed refusal, never a silent mis-parse.**
awk on the shell side is GNU awk, whose ERE dialect carries extensions POSIX does
not: `\y` `\<` `\>` `\B` `\w` `\s` and the backreference forms. `compile` returns
`EreError` for any escape or construct outside the POSIX ERE grammar, and each
member turns that into **exit 2** naming the offending pattern, the knob it came
from, and the extension. Three constructs are refused for the same reason rather
than resolved by picking a dialect: a backslash *inside* a bracket expression
(POSIX reads a literal, GNU an escape), a collating-symbol or equivalence-class
bracket, and any escape outside the specials list. This is §Fail-closed contract
applied to a parser rather than to a subprocess, and the failure it guards
against is observable: a marker knob spelled `\yformerly\y` makes GNU awk warn on
stderr, treat `\y` as a plain `y`, and report a **clean** verdict off a scan that
never matched what the consumer meant. The live vocabularies use no extension, so
the refusal is a guard for consumers rather than a change to this tree.

**The acceptance oracle is a differential run against the shell's own awk.** A
hand-written regex engine is the one component where authoring the tests and the
implementation from the same understanding proves nothing, so `ere.rs`'s unit
arm stands up a throwaway corpus and compares `is_match` against `awk '$0 ~ p'`
and `find` against `awk 'match($0,p){print RSTART, RLENGTH}'`, byte for byte,
over a generated pattern-and-subject cross product. The pattern crosses through
`ENVIRON` rather than `-v`, so awk's own string-escape pass never rewrites it
before its regex compiler sees it, and the oracle runs at `LC_ALL=C` to hold both
sides to the same byte semantics. The generator covers the constructs this tree's
vocabularies never exercise — intervals, nested alternation under a quantifier,
negated bracket ranges, anchors inside groups, the character classes — because
those are exactly the branches no fixture pair and no live-tree run reaches. It
is criterion 2's constructed-scenario form applied to a *mechanism*, the same
move the canon-kit cohort made for the default walk. The arm runs under
`check-crate-arms`, so a divergence is a commit-time red.

**The boundary the cohort applies: a pattern the kit owns is hand-compiled; a
pattern a consumer supplies goes through the engine.** The prefix strip and the
inline-code stripper `check-manifest-temporal` applies are substitutions over kit
literals and port as direct code, which is why the API owes no `replace`.

**The boundary is refined, not reversed, by the first member to port *after* the
engine landed.** "Hand-compiled" is a claim about the **substitution**, which
stays the port's own cheap work; it was never a claim that the port must also
hand-write a *matcher* for the pattern being substituted. When
`check-queue-prose-precondition` ported, hand-compiling
`(once|when|after)[^.,;]*(landed|shipped|merged|resolved|completed|was [a-z]+ed)`
would have meant writing an alternation, a negated-class gap and a sub-matcher by
hand with leftmost-longest arbitration, in a gate module, beside a compiled
matcher that already answers it — roughly forty lines of exactly the code the
engine exists to stop anyone writing. So a kit-owned pattern **compiles through
the engine like any other**, and what the member owns is the substitution loop
around it: a member ports against the engine that has since landed rather than
around it. One
deliberate exception buys the span path a production reader:
`check-install-claim`'s heading extraction routes through `find` rather than a
second hand-written scanner, so a precommit-tier gate exercises the span API on
every invocation instead of leaving it alive only in unit tests.

**The provenance seam, ruled, because this cohort is where it bites.**
`native/src/ere.rs` carries **no vocabulary whatever**: not a transport id, not a
disclosure class, not a temporal marker. An engine sized to a grammar rather than
to a corpus cannot encode one project's terms, and the pressure to shrink it
toward the patterns this tree happens to configure is exactly the pressure that
would. The transport vocabulary, the disclosure vocabulary and the temporal
marker set stay consumer config; the bridged arrays below change the *transport*
of two of them and not their ownership. The declaration regexes, the heading and
fence patterns and the inline-code stripper are grammar the kit owns, carry no
project term, and are the kit literals the boundary above leaves hand-compiled.
The check this reduces to: a `grep` for any transport or disclosure id across
`native/src/` returns nothing.

**The criterion-5 price, measured against the post-cohort registry.** The
binary-less leg installs the `prose` profile from an artifact-free payload and
reports the roster that profile loses with the binary: **seven members**,
declared in the consumer's own registry, against six before this cohort. The
roster grew by exactly one — `check-manifest-temporal`. Its two cohort siblings
are absent from that profile's registered set, because a gate is registered
**where its surface exists** and this profile configures neither a transport
vocabulary nor a disclosure one. That is the measurement's honest limit rather
than a discount: a consumer who configures either surface registers the matching
member and loses it too, so the priced roster is this profile's and not every
consumer's. The judgment the criterion leaves to a cohort: the roster is
acceptable, because all three members assert over governed **markdown prose**
whose other readers stay shell-side, and no class of adopter-authored content
goes unheld by their omission.

**Parity, proved while both implementations existed.** Assertion A forbids a
`<name>.sh` and a `<name>.gate` in one resolve dir, so each comparison ran with
the shell gate in place and the descriptor staged elsewhere, and the descriptor
landed as the script was deleted in one motion. Per member: the fixture pair, the
live tree, and a constructed differential corpus for the arms neither reaches —
both clean skips per claim member, an empty governed-doc set, zero and two
declarations, an id outside the vocabulary, `~~~` fences, a heading inside a
fence, earliest-match-wins-per-section, the declaration line as non-evidence,
path-exempt and section-exempt windows, inline-code stripping, and marker
patterns using intervals, anchors and character classes. Byte-identical, exit
codes included.

### The diff renderer

**The crate's one rendering of `diff`'s normal format.** `native/src/diff.rs`
carries `normal_diff`: a pure function over two line slices returning the
normal-format hunk sequence — the `NcM` / `NdM` / `NaM` range headers and the
`<` / `---` / `>` body lines the external `diff` prints with no options. It was
`check-lifecycle-registration`'s private mechanism until the generated-projection
freshness members that will each need one at their own port made it shared, and it
is a module ahead of its second consumer for exactly that reason: what
justifies it is the cross-kit reach it removes from a later port's path, not
generality claimed on its behalf.

**It carries no vocabulary and no report policy**, the §The POSIX ERE matcher
scoping applied to a renderer. It reads no knob, takes no path, and knows nothing
about what the two slices it is handed represent.

**The renderer is uncapped; the cap belongs to the caller.** `normal_diff`
returns every hunk. A renderer that truncates is a reporter, and the two roles are
separated because the crate's one live caller
(`native/src/gates/lifecycle_registration.rs`, its only consumer today) wants the
whole sequence while the shell gates a later port replaces legitimately do not.

**The cap's value is the shell family's, and a port carries it rather than picking
it.** Every generated-projection freshness gate that renders a diff at all caps
its stale report at one literal, `diff <(...) | head -20`, and that literal is the
value's only live owner.
Criterion 2 proves a ported member byte-identical to its shell original, so a port
that re-picks the cap — or drops it, taking the uncapped return straight to
stdout — diverges on precisely the cases a fixture does not build: a planted `bad/`
difference is small, and the live tree is where a report first exceeds the cap. The
rule: **the first freshness member to port lands the cap as one crate constant with
its first reader in the same commit**, and every later member of the family reads
that constant instead of repeating the literal.

That constant is deliberately **not** landed ahead of its reader, and the omission
is a rule rather than an oversight. `check-crate-arms` runs clippy at `-D warnings`,
where a constant no code reads is `dead_code` in this bin crate; the only ways to
hold one early are a lint suppression the crate has never used or a caller invented
to read it, and both are worse than stating the contract here and landing the value
at the port that first needs it.

**§The consumer remainder cohort discharged the rule, and the constant is
`STALE_REPORT_CAP`** in `native/src/diff.rs`. It arrived with the two readers that
cohort brought — `check-value-rollup-fresh` and `check-trajectory-fresh`, the two
members carrying the `head -20` literal — so a later family member reads the
constant rather than the literal, and the value has one owner again. **Applying it
is the caller's, and the caller is a reporter rather than the renderer**:
`native/src/fresh.rs` carries the family's shared shape — the emitter spawn, the
command-substitution read, `diff`'s own line model, and the one site the cap is
applied at — while `normal_diff` stays uncapped and pure. That separation is this
section's own rule taken literally: the module that truncates is not the module
that renders, and putting the cap in the renderer would have been the first
convenient way to lose it.

**The renderer never spawns, and that is a contract rather than an accident.**
`diff` is on `GATE_SDK_PROGRAM_FLOOR` (§lib/gate.sh), so criterion 7 clears a
ported form reaching for `Command::new("diff")` and a session under parity pressure
has every reason to reach for it. It is refused: `native/src/proc.rs` is the
crate's one sanctioned spawn site, an LCS walk needs none, and a spawned renderer
would reinstate per member exactly the external-program dependency
TRAJECTORY.md objective 1 exists to collapse. Stated here so the cheapest wrong
implementation is refused by a rule rather than by whoever reviews that port.

### The settings cohort, and the crate's first dependency

**Two members: `check-settings-paths` and `check-settings-pins`** (context-kit),
and what the cohort buys is a JSON reader. It is named for the precedent it
actually sets — the first dependency — and deliberately **not** on §The POSIX ERE
matcher's engine pattern: that engine had to be **built**, and paying it against
three members was what justified the override there. A JSON reader did not have
to be built. What this cohort paid was a `cargo add`, and a later reader must not
cite it as precedent for *"what it buys is the engine"*.

**Selection evidence, from a `--group` run at the cut** (2026-08-14):
**104 members scanned, 47 groups formed, 0 undecidable, 42 already ported** — 62
remaining shell. Group sizes are 1×14, 2×2 and 44 singletons, and the 14-member
group is operator-ruled not a cohort (§The canonical-spec cohort: `fail_closed`
derives no corpus), while each 2-member group holds one member that stays shell.
So the **largest takeable derivation group is one member**: the selection rule's
size arm is exhausted, and this cohort is selected under the documented override
— *a cohort that retires a blocker several later cohorts are queued behind
outranks a larger one that retires none*. The undecidable count is the bound on
that claim, and it is zero.

**The two members share no `--group` key, and that is recorded rather than
smoothed over.** `check-settings-paths` cut as `libs=fail_closed globs=*.sh` and
`check-settings-pins` as `libs=gates_list_members globs=-` — two singletons. What
they share is the **`couples=` corpus**, `.claude/settings.json`, and the
criterion-7 `jq` blocker; the tool's key is (libs, globs) and does not see either.
Since the cohort was selected by the override rather than by grouping, a shared
key was never what justified it — but a later reader comparing this section to the
tool's output would otherwise conclude one of them was wrong.

**The dependency, and the bar it cleared.** `native/Cargo.toml` takes
`serde_json` and the cohort implements no JSON parser. The crate is under no
no-dependency prohibition and never was: the constraint is the **adopter's** —
their machine requires only git and the prebuilt binaries — and TRAJECTORY.md
§The objectives' objective 4 governs what an adopter installs, saying nothing
about a build graph no consumer ever receives, resolves or compiles. The bar a
candidate clears, a bar rather than a precedent: it performs **no filesystem
walk** (the condition `native/src/walk.rs`'s own assertion states), spawns no
subprocess, opens no socket, its MSRV is at or below the crate's floor or the
floor moves deliberately, and its **transitive** set is small, enumerable, and
admitted under the same clauses. The resolved graph is **11 packages** — five
feature-activated (`serde_json`, `serde_core`, `memchr`, `itoa`, `zmij`) and six
carried by the lock as unactivated optional deps — each named with its admitting
clause in `walk.rs`'s allowlist, which is the machine-held form of this bar rather
than a second list beside it.

**The floor moved, and the measurement is what moved it.** Taking the dependency
raised the crate's MSRV from **1.56 to 1.71** — the binding crate is `zmij`, and
it binds through the activated set, so no feature trim avoids it. The alternative
was priced: pinning a pre-`zmij` `serde_json` would hold 1.56 at the cost of
freezing a dependency at an EOL version, which contradicts the maintained-
dependency premise the bar rests on.

**An MSRV bump is a `check-crate-arms` input, which no surface said.** Clippy
suppresses a lint whose suggested API postdates the declared `rust-version`, so
raising the floor **un-suppresses lints against unchanged code**: this bump
surfaced four findings in modules the cohort never edited. Confirmed by controlled
experiment rather than inferred — HEAD's tree with only the `rust-version` line
changed reproduces exactly those four, and zero at 1.56. A later floor move
budgets for a clippy pass it did not write, and the four were fixed here.

**The pin-path layer is what the cohort authors, and it is the only new surface.**
`Path::compile(&str) -> Result<Path, PathError>` and its evaluation over a
`serde_json::Value`, with exactly two consumers —
`native/src/gates/settings_pins.rs` and `native/src/gates/settings_paths.rs`
(which takes no path expression at all: its allow-array access is kit-owned and
hand-compiled, §The POSIX ERE matcher's boundary applied unchanged). No filter
language, no mutation, no second grammar. The grammar, its refusals, the
structural comparison and the two deliberately-preserved shell semantics are
context-kit/SPEC.md §check-settings-pins'.

**The acceptance oracle is a differential run against the shell gate's own
verdict**, generated over path shapes rather than document shapes: with a
dependency the arm proves the layer this cohort authors, and a generator varying
documents would spend its budget re-testing `serde_json`. It compares verdicts
rather than `jq`'s rendered stdout, because that rendering is version-dependent
and an oracle pinned to those bytes would fail on a contributor's newer `jq` while
the gate was correct. It models the **gate**, not the tool: the shell refuses a
path not opening with `.` before `jq` ever sees it, and the arm earned that
distinction by catching a real divergence — `jq` reads a leading `["k"]` as an
array *literal* and returns `["k"]`, so a grammar admitting one would have made
the compiled side answer a question `jq` was never asked. The arm needs `jq` at
**contributor** time, which touches criterion 7 not at all and adds nothing to
this repo's floor while the two held members below keep `jq` in the battery.

**Criterion 5, priced by measurement.** Both members are `install: on-surface`, so
the lifecycle-kit cohort's precedent **predicted** a binary-less residual growth
of zero. Measured with `installer_smoke`'s binary-less leg after this cohort's own
commit: **12 members omitted and declared**, and the prose profile's descriptor
set is untouched by the cohort, so growth is **zero** and the prediction holds.

**What that measurement does and does not reach, because the leg is scoped.** The
binary-less leg runs the **prose** profile, which vendors `gate-sdk` and
`canon-kit` — not `context-kit` — so it structurally cannot see either member of
this cohort, and its unchanged 12 is evidence that nothing *else* moved rather
than evidence about these two. What carries the zero on `delegation` and `full`,
which do vendor `context-kit`, is the `on-surface` disposition alone: a member no
`init` seeds cannot be a member an artifact-free `init` loses. That limit is
restated rather than banked — an adopter who later brings
`.claude/settings.json` into existence on a binary-less host **does** lose both
members. A cohort whose members land in a profile the binary-less leg actually
installs gets a real measurement where this one gets a scoped one, and should not
read this section as precedent that the leg covers every port.

**Two members a mechanical reading puts in are excluded, each with its ground.**
`check-memory-off` is **held on criterion 2**, named and not ported: its
`--fixture <dir>` arm is a different code path from its live arm, whose corpus is
the harness memory directory under `HOME` and is not in the tree at all, so the
pair proves nothing about the part being ported. What it owes is criterion 2's
**constructed-scenario** discharge, and that is the cohort taking it. Its blocker
is the oracle, not the dependency, so this cohort's reader does not unblock it.
(§The third budget batch is the cohort that took it, and this clause is why it
had to: retiring `jq` could not have retired the hold, so the discharge is what
paid for the member and the reader it inherits from here is the dividend.)
`check-installer-no-deps` is **excluded with cause**: as the first
`scripts/`-declared gate to enter `native/` it would drag in the tranche's
unanswered first-mover questions — whether a consumer-declared member earns a
conservation row, how assertion B's owner column reads a member no kit ships —
which are `consumer-gate-port-disposition`'s design work and are budgeted there.
Its *other* original ground, that membership beyond what proves the engine adds
risk without payoff, is materially weaker now the engine is a dependency rather
than a build; it is recorded as **retired** rather than quietly kept, and the
exclusion does not rest on it.

**The honest claim about `jq`, which is the one this cohort must not overstate.**
After the cohort, `jq` is retired from the battery **but for those two members**,
both with named owners; after §The third budget batch, but for
`check-installer-no-deps` alone. It is **not** retired from the shipped install path at
all — `installer/lib/` shells to it. *"The cohort retires jq"* is false in both
directions. On a machine without it the verbs **refuse, naming the program**
(installer/README.md §Requirements); silent degradation on that path was
`installer-jq-silent-degradation`'s subject and that entry is discharged.

**The enforcement-map port later closed one of those two**, and it is recorded
here rather than beside the port because this is where the claim is stated. That
emitter read the harness settings hooks through two `jq` programs; ported, it
reads them through the crate's own JSON dependency. The retirement is a
**dividend of a port that was argued on other grounds**, not a unit bought to
chase it — which is the shape §The port-candidate criteria says an
external-program dependency should take: engineering work the port owes rather
than an exclusion it may claim. The boundary above is unmoved in the direction
that matters: the *shipped install path* still shells to `jq`. Its silent-failure
half closed separately and in the same iteration, above.

**The provenance seam, ruled here because a settings reader is where it bites.**
The pin-path layer carries no settings key, no pin path and no permission
vocabulary: the pins manifest and the settings file are consumer config, and the
grammar is sized to a grammar rather than to a corpus, so there is nothing for a
project term to attach to. The permission-entry grammar `check-settings-paths`
hand-compiles is a **public harness format**, the same class as the `# graph:`
manifest grammar the crate already parses. The seam reaches the dependency too,
and the reading is short: a general-purpose JSON parser is **grammar, not
vocabulary**, so it can no more carry a project term than `ere.rs` could. What the
seam still forbids is a *consumer-shaped* dependency — one selected because it
encodes some tree's terms — and the bar above excludes that class by construction.

### The declaration cohort

**Three members: `check-release-bump`, `check-tightened-gates-grammar` and
`check-tightened-gates-note-parity`**, and what the cohort buys is the
release-note declaration family off the shell substrate. Named for what it buys,
on §The POSIX ERE matcher's precedent rather than for a group key — the group key
is precisely what did *not* select it. All three are **consumer-declared**, so
this is also `consumer-gate-port-disposition`'s first tranche, 3 of 13, and the
first-mover design every later tranche inherits is
§check-gate-substrate-parity's assertion-B owner clause rather than this
section's.

**Selection evidence, from a `--group` run at the cut** (2026-08-15): **104
members scanned, 45 groups formed, 0 undecidable, 44 already ported** — 60
remaining shell. Group sizes are 1×14, 2×2, 2×2 and 42 singletons, and the
**largest takeable group is one member**: the 14-member group is operator-ruled
not a cohort (§The canonical-spec cohort: `fail_closed` derives no corpus), one
two-member group holds `check-install-disposition`, which the table above keeps
on shell, and the other holds `check-docs-render-fidelity`, held on criterion 7.
So the size arm is **exhausted a second consecutive time** — the ninth cohort was
the first — and this cohort is selected under the documented blocker-retiring
override, never by member count. The undecidable count is the bound on that
claim, and it is zero.

**The tool split one derivation into three singletons, and the session rejoined
them — which is the addition this cohort makes to the recording contract.** The
members cut as three separate keys: `libs=collect_dispositions,decl_section_bullets,fail_closed,section_bullets globs=*.md`,
`libs=decl_record_tokens,decl_section_tokens,fail_closed globs=*.md` and
`libs=decl_section_tokens globs=*.md`. The **adjudicated union key** is
`libs=collect_dispositions,decl_record_tokens,decl_section_bullets,decl_section_tokens,fail_closed,section_bullets globs=*.md`
— one 59-line library and one walk. Recording the union is what makes the section
legible: without it an override cohort whose members the tool reported as three
singletons reads, to a later selector, as three coincidences. The adjudication is
the selecting session's by the rule above, read in its mirror image — a group
whose members' corpora visibly diverge is a finding, and so is a derivation the
key split.

**Criterion 6 leaves the library dual, and what that owes is a standing oracle
rather than a port-time proof.** `bin/upgrade-smoke.sh` survives the port as
`lib/declaration.sh`'s only remaining caller, so neither the duplication-absent
road nor the deleted-original road is available and the disposition is
queue-kit's `lib/queue.sh` one, taken by the same mechanism. The arm, the test
and the corpus that carries every branch are §lib/declaration.sh's.

**The version comparator is defined over a stated grammar, and a token outside it
is a loud refusal.** The shell form ordered versions with `sort -V` in four
places; the compiled form orders `<major>.<minor>.<patch>`, each a run of ASCII
digits, field-wise numeric, with the row form tying on the path's byte order, and
refuses anything else at exit 2 naming the token, its source and the grammar.
**The ground is that a faithful port and a correct one are different programs
here.** Probed: `sort -V` orders `1.0.0` **before** `1.0.0-rc1` and semver orders
it **after**, so reproducing `sort -V` bakes a prerelease ordering into a gate
whose whole subject is a semver line, while implementing semver instead is a rule
change smuggled through a substrate change. Refusing is the only disposition that
neither invents a rule nor ships a known-wrong one — §The POSIX ERE matcher's
refusal shape applied to an ordering rather than to a parse. It forecloses
nothing: the prerelease path stays exactly as open as `scripts/pack-installer.sh`
already leaves it, and docs/install.md §Versioning now names where the ordering
ruling is owed. Measured against this tree, all 23 live `release:` keys are bare
`vX.Y.Z` and the disposition file carries no data line, so the refusal is a guard
for the future rather than a change to this repo today. The `sort -u` inside the
disposition collector is **not** version ordering — it is a byte sort with dedupe
over whole lines and ports as one, named because two sorts four lines apart in
one function is exactly how a port fuses them.

**A rejection case cannot pin a refusal, so the arm is pinned where refusals
already are.** §run-gate-tests makes a `bad/` case exit 1 by contract and a
refusal is exit 2, so no fixture case can hold this arm — the pin is
`scripts/gate-tests/check-release-bump.test.sh`, alongside that gate's other
exit-2 cases, and it covers **both** producers, a note's `release:` key and a
disposition data line, because a refusal naming the wrong file sends its reader
hunting.

**A cohort port is one commit, and that is a construction rather than a
preference.** §check-gate-binary-fresh's stamp is computed from `git ls-files`,
so a partial-path `git commit -- <paths>` — which swaps in a temporary index of
HEAD plus the named paths — recomputes the crate's manifest without the sources
the partial commit leaves out, and a binary built from the whole tree can never
match it. Splitting the modules from the registry, the descriptors or the `.sh`
deletions therefore reds rather than staging cleanly, which is the gate refusing
to record a crate tree nothing was built from. A session budgeting a cohort into
several commits budgets around this.

**Parity was proved while both implementations still existed**, which assertion A
makes the only possible order: each member's script and its subcommand run over
every fixture case, over the live tree, and over the edge roots (an absent
directory, an empty posts dir, a one-note tree), compared on stdout, stderr and
exit code. Byte-identical on all of them, with **exactly one divergence, and it is
the designed one** — a prerelease token, where the shell prints a clean line
carrying a mis-ordered pair and the compiled form refuses.

**Both subprocess reads go through the crate's one spawn site**, and a unit test
leaves a gate module unable to construct a child at all (§Fail-closed contract).
The tag probe's non-zero status is the *no such tag* branch, a verdict rather
than a failure, so it is read through the exit-code accessor that section added
for exactly the child that grades its own outcome by its exit code. The historical
`git log`'s added lines are extracted with a pattern the gate itself owns, so it
is hand-compiled rather than routed through the engine — §The POSIX ERE matcher's
boundary, unchanged.

**Two conflations are preserved rather than repaired, because a port proves
parity and does not fix rules.** The first is inherited: `check-release-bump`
silences its historical `git log`'s stderr and returns success regardless, so a
tree where git cannot answer derives its deferral floor from the live file alone.
The compiled form reproduces it and the reasoning is recorded rather than left as
an accident of translation — the branch is unreachable in the live tree, which
always has a repository, and in the fixture pair, which supplies the file, so
there is no case that would prove a change correct. The second the port
**found**, and it is the shell library's: both arms leak already-resolved tokens
into the refusal output (§lib/declaration.sh). Both are filed as gaps so the debt
is costed rather than implicit.

**The three declare no knobs, and the `.workflow/` asymmetry that exposes is
preserved rather than repaired** — because repairing it needs an answer this
tranche does not have. Both `.workflow/` defaults are hardcoded in the gates while
`bin/upgrade-smoke.sh`, a reader of the *same* declaration file, resolves the
directory through `GATE_SDK_WORKFLOW_DIR`. Honouring the knob in the compiled form
would make it a **declared** knob, and the config bridge resolves a declared knob
against exactly one kit's library — attributed from the knob's own name, never
from the declaring gate's location (§lib/gate.sh). A knob the *consumer* owns is
therefore resolved against a kit that does not define it, and the bridge's
undeclared-knob refusal fires on every invocation. That is the config-bridge question
`consumer-gate-port-disposition` must answer for the first knob-declaring member
of its tranche; it is **named here and deliberately not answered**, because
answering it inside a port whose members need no knob would be designing against
no case. Keeping the literals keeps `gate_command` on its zero-knob path and
leaves the asymmetry exactly as visible as it is today.

**Assertion C's derivation was re-run at the cut, in both directions: zero
transitions.** The declaration paths moved from `<gates-dir>/<name>.sh` to
`<gates-dir>/<name>.gate`; every member reaching these three through a
`scripts/*.sh` glob also matches other surviving `.sh` declaration paths, so none
loses sensitivity, and the members carrying a bare `*.gate` token already matched
dozens of descriptors, so none gains it. A row the derivation stops selecting is
not removed on that ground alone; only the newly-selected direction can fail, and
it selected nothing.

**The other readers of the narrowed corpus were enumerated by red condition
rather than by subject**, because *a narrower corpus can only remove violations*
is false for three ordinary shapes and this narrowing met all of them
(canon-kit/SPEC.md's causal-completeness point 5). The **inverse** shapes:
assertion A reds on a descriptor and a script *coexisting*, so a half-done port
violates it and the finished one satisfies it; `check-exec-bit`'s `.gate` arm is
unconditional and reds on a descriptor committed **executable**, the opposite of
its usual condition. The **reds-on-empty** shape: `check-shellcheck` exits 2 on an
empty target set, re-checked at build rather than inherited, since the gates
directory retains many `check-*.sh` and the ground is a count.
`check-gate-fixture-coverage` reds on a registered member with no pair, and all
three pairs are retained — deleting a script and orphaning its pair are one action
apart. `check-reads-couples` reds when observed walk roots are not a subset of
declared ones, answered here by `--reads` with a single `?`: the walked directory
is relocated by the gate's own positional argument, so declaring the live default
would red unit test A the moment a fixture case ran. Two readers are
**structurally out of reach, measured rather than assumed**: `check-readme-roster`
and `check-install-disposition` both sweep `gate_kit_roots` only, so neither names
a consumer-declared member before or after, in either direction. The general fact
the first exposes — that no roster reader covers a consumer-declared gate at all —
is a standing coverage question filed to the gap inbox rather than answered inside
this cohort. `check-settings-paths` reds on a committed permission grant naming a
`.sh` path that does not resolve, and re-verification at build found none naming
these: the two grants that could have carry a `*` in the command token and are
skipped by the predicate. One non-red consequence is recorded so it is not
mistaken for coverage — those wildcard grants silently stop covering this
cohort's members, which do not run as scripts. `check-docs-cmd` reddens on
§lib/declaration.sh's caller roster, where the deleted paths were named, and the
roster rewrite this cohort already
owed is what clears it; that is real signal, exactly as the disposition table
predicts for it. `check-measured-claim` reds because its oracle counts registry
members resolving to a `.gate`, so the marked literal in docs/install.md moves
with the port — and the generated pre-commit hook bakes the same value, which is
why the hook regen is owed with the doc edit rather than after it.

**What this cohort paid that the last four did not is the callers, and it is the
lifecycle-kit cohort's lesson recurring on a consumer's own gates directory.**
Both bespoke `scripts/gate-tests/*.test.sh` behind these members named their gate
by **script path**, so the port would have left them invoking a deleted file; both
now reach it through `gate_run` (§run-gate-tests), which is what makes a
behavioral test survive a substrate change. A tranche porting consumer-declared
members enumerates that directory's `*.test.sh` before the cut, exactly as a
selector reaching a kit that ships `bin/` or `smoke/` does.

**Criterion 5, priced by measurement, on a prediction that was structural rather
than empirical.** All three members are consumer-declared, so they sit in no kit's
`checks/`, `init` can never seed them, and no adopter has ever had them — a
residual counts what a consumer *loses*, and this cohort takes nothing from any
consumer. That is a stronger ground than the ninth cohort's `on-surface`
argument, and it still does not discharge the criterion: N members each
individually runnable is not a measurement. Measured with `installer_smoke`'s
binary-less leg after this cohort's own commit, from a clean checkout reached by
path: **12 members omitted and declared**, the same figure the ninth cohort
measured, so growth is **zero** and the cohort lands on that finding. The
consumer-declared ground is restated here rather than banked, because it is what
makes the zero predictable rather than lucky. The judgment was fixed in advance,
and a non-zero would have been a finding about the *measurement* as much as about
the cohort.

**Two exclusions, each with its ground — and both are now discharged.**
`check-installer-no-deps` was ruled past this iteration at scope and was **not**
rescinded there; it remained the named cheapest single-gate first mover for a
later cohort, with its `jq` reader already paid. The other ten consumer-declared
members were **sequencing, not exclusion**, on the standing rule that every held
member has port work owed rather than a waiver — three held on an unported
emitter and one on an undetermined criterion 7. **§The consumer remainder cohort
takes all ten**, folding the first mover in with them, so both clauses are spent
rather than standing.

**The config-bridge question this section named is re-pointed rather than
answered here, and the re-pointing corrects which member owes it.** The text
above attributes it to `check-installer-no-deps`; probed at the remainder
cohort, that gate reads **no** knob at all — its only environment read is the
`GATE_SDK_ROOT` bootstrap every gate script carries to locate `lib/gate.sh`,
which has no compiled counterpart because a compiled gate sources nothing. The
question belongs to **the first consumer-owned knob name**, and no member of the
thirteen has one. What the remainder cohort did meet is the hazard's *other*
door, and it is worth separating from the one this section described: the
bridge's undeclared-knob refusal fires not only on a name that misattributes to
the wrong kit, but on a name that attributes to the **right** kit whose library
does not `declare` it. `GATE_SDK_QUEUE_FILE` carried gate-sdk's prefix and was an
environment-only override every reader spelled inline as
`${GATE_SDK_QUEUE_FILE:-…}`, so prefix attribution succeeded and `declare -p`
found nothing. The repair is the one §Layout and configuration already states
twice — resolve the knob in `lib/gate.sh` rather than inline at its readers —
applied a third time, and it is why *a knob whose name carries a live kit's
prefix resolves* is a necessary and not a sufficient condition.

### The consumer remainder cohort

**Ten members: the whole remainder of this repo's own gates directory** —
`check-docs-kit-parity`, `check-docs-mirror-fresh`, `check-docs-nav-reachable`,
`check-install-toolchain`, `check-installer-no-deps`, `check-kit-ref-liveness`,
`check-npm-publish-spec`, `check-release-channel-parity`,
`check-trajectory-fresh` and `check-value-rollup-fresh`. They ship as descriptors
dispatching to the binary, the ten shell scripts they replace are deleted, and
`scripts/` keeps **no** `check-*.sh` at all. This completes
`consumer-gate-port-disposition` 13 of 13 and retires it.

**Operator-ruled 2026-08-15 at scope: the remainder, all ten**, on the
destination ruled 2026-08-14 (TRAJECTORY.md §PRIORITY DIRECTIVE — the port
track's sequence, its consumer clause). The tranche's ground is **not** a member
count: `--group`'s size arm is exhausted a **third** consecutive time, so the
live ground is the tenth cohort's documented blocker-retiring override (§The
first cohort, and the rule that selects the next) plus the fact that the
first-mover design is already paid, so every member inherits it and marginal cost
per member sits at its floor.

**Selection evidence is cited rather than re-bought.**
`.workflow/survey-record.md`'s 2026-08-15 scope block answers *which gates
compose this cohort*; its witness was re-run at the authoring stage and holds —
the corpus is unmoved since the recorded revision but for the queue, and
`bash gate-sdk/bin/port-blockers.sh --group` reports the same verdict the finding
was written against: **104 members scanned, 0 undecidable, 47 already ported, 57
still shell**. The undecidable count is the bound on the claim, and it is zero.

**Criterion 7's `?` was scanner blindness, and tracing it is what made the
tranche ten.** `port-blockers.sh` reported `check-docs-kit-parity ?
… (command-position $WRAPPED, default unresolvable)`: the command word there was
an array populated at run time by `gate_command`, which no static scan resolves.
Traced by hand, the runtime program set was `{awk, bash, the crate's own
binary}` — the first two on `GATE_SDK_PROGRAM_FLOOR`, and the wrapped member
`check-kit-registration` already ported, so the third element was the binary this
cohort ports *into*. Criterion 7 clears. Scope's fallback — *if a manual
criterion-7 check does not clear it the tranche is 9* — was therefore **executed
and spent**, not declined, and it is recorded in that shape because a later
reader meeting the fallback needs to know which.

**The composition is a first: a compiled gate whose rule is another gate's
verdict.** `check-docs-kit-parity` wraps `check-kit-registration` and must keep
doing so across the substrate boundary. The shell form reached the wrapped member
through `gate_command`, captured its combined output and re-framed it by exit
code — 2 passes the wrapped diagnostic through to stderr unchanged, non-zero
becomes this gate's own headed failure on stdout, 0 falls through to the nav-block
sweep. **The compiled form calls the module and does not spawn itself**: both
members live in one binary, so spawning would put a gate's own process through
`native/src/proc.rs` for no reason and would make a gate's verdict depend on the
binary being executable from a path it happens to know. The port therefore owes a
**capturing entry point** — the existing `run(&[String]) -> i32` retained
unchanged as the dispatch signature, the body moved behind a form writing into
caller-supplied sinks — and the wrapper calls that form. Letting the wrapper call
`run` directly and leaving the wrapped output on stdout was **ruled out**: it
passes a `good/` fixture and diverges on every failing case, where the shell form
prints one headed report and the compiled form would print two unheaded ones.

**Two knob declarations are what the members execute, and that is not what their
scripts spell.** The amendment predicted nine empty knob slots off a probe that
counted `${KNOB:-default}` reads written in the gate scripts; the executed shape
is **eight empty and two carrying three each**, because the standing rule is that
a member declares every knob a shared derivation it calls is computed from.
`check-kit-ref-liveness` reaches `gate_kit_roots_rel` and `gate_path_pruned`
beside its own queue-file read, so it declares `GATE_KIT_ROOTS_REL`,
`GATE_PRUNE_DIRS` and `GATE_SDK_QUEUE_FILE`. `check-docs-kit-parity` reaches the
wrapped rule **in-process**, so its dispatch must carry what `gate_command` used
to resolve on the wrapper's behalf: the wrapped member's whole knob set. The
correction is recorded rather than quietly implemented, because the arithmetic is
what a later selector would inherit — a wrapper's knob cost is the wrapped rule's,
and an in-process call is what moves that cost onto the wrapper.

**One knob's resolution had to be repaired before it could be declared**, and it
is the second door onto §The declaration cohort's hazard rather than a new one —
that section's own paragraph carries the finding.

**Three members port ahead of their emitters, and the cost is stated rather than
absorbed** — §The generated-projection freshness family carries the accounting,
the supersession and the honest zero.

**One member's fail-closed repair came due at the port, and it is this cohort's
single designed divergence** — `check-docs-mirror-fresh`, at §The
generated-projection freshness family's table row.

**`check-installer-no-deps` drops `jq` for the crate's existing JSON reader.**
The gate's whole predicate is *does this package declare any of three dependency
fields, or any of three install-time lifecycle script keys* — a `has(key)` test
over one object and one nested object, which `serde_json` (§The settings cohort,
and the crate's first dependency) expresses directly. The `command -v jq` guard
and its fail-closed exit go with it: a linked parser cannot be absent from
`PATH`, so the branch has no compiled counterpart. The gate's own semantics are
preserved exactly, including the one they turn on — **the field's presence is the
finding, not its emptiness** — and so is a detail a tidier port would have
"fixed": the shell indents the *capture* rather than each finding, because
`printf '  %s\n' "$findings"` takes one argument, so only the first line carries
the lead.

**Assertion C's derivation was re-run at the cut, in both directions, and this is
the widest single move it has seen** — ten `<gates-dir>/<name>.sh` paths becoming
`<gates-dir>/<name>.gate` in one commit. `check-value-rollup-fresh` is itself in
§Meta-gate conservation's *survive unchanged* row and is itself a member here, so
its row records that it is now `.gate`-dispatched and that its `scripts/*.sh`
coupling covers **no** registry member's declaration path any more while
`kit:*.sh` still does. The other readers were enumerated **by red condition,
never by subject**: `check-gate-fixture-coverage` reds on a registered member
with no pair and `check-shellcheck` exits 2 on an empty target set, so neither is
monotone in the violation set and both were re-checked by **running** them, not
by inspection. Every fixture pair is retained.

**Every fixture pair is retained and re-pointed, and two members' parity could
not be bought from a pair at all.** Each member's `gate-tests/<name>/good/` and
`bad/` case directories are unchanged; the two bespoke
`scripts/gate-tests/*.test.sh` that named their gate by **script path** now reach
it through `gate_run` (§run-gate-tests), which is the tenth cohort's recorded
lesson about a consumer's gates directory. `check-trajectory-fresh` and
`check-value-rollup-fresh` are two of the five family members whose pairs steer
off the live emitter through the emit-source positional, so their criterion-2
parity was bought by a **live-tree run** with both implementations still present
— and deliberately by a *stale* one, since a clean live run exercises neither the
diff renderer nor its cap.

**Parity was proved while both implementations still existed**, which assertion A
makes the only possible order. Every member's script and its subcommand ran over
every fixture case, over the live tree and over nineteen edge roots, compared on
stdout, stderr and exit code: **byte-identical, with exactly one divergence, and
it is the designed one** (the mirror member's unreadable-`docs` refusal). Five
further constructed scenarios covered the arms no case reaches — both family
members' stale report against the live emitter, each long enough to prove the
cap, and the mirror member's missing and orphan arms.

**Criterion 5, priced by measurement on the tenth cohort's structural ground.**
All ten members are consumer-declared: they sit in no kit's `checks/`, `init` can
never seed them, and no adopter has ever had them, so the cohort takes nothing
from any consumer and the prediction was **zero** growth. N members each
individually runnable is not a measurement, which is why the ground does not
discharge the criterion; measured with `installer_smoke`'s binary-less leg from a
clean checkout reached by path, after this cohort's own commit: **12 members
omitted and declared**, the same figure the ninth and tenth cohorts measured, so
growth is **zero** and the cohort lands on that finding. The judgment was fixed
in advance, and a non-zero would have been a finding about the measurement as
much as about the cohort — which is what makes the third identical reading worth
recording rather than skipping. The consumer-declared ground is restated here
rather than banked, because it is what makes the zero predictable rather than
lucky: a residual counts what a consumer *loses*, and no consumer ever had these.

### The twelfth cohort

**Two members, selected by the size arm's group 1 and sharing no blocker.**
`check-close-surfaces` and `check-queue-prose-precondition` were `--group`'s
first group at the cut — both `libs=fail_closed globs=-`, both clearing criteria
2, 3 and 7, neither tripping criterion 4. The arm selected on *shape*, and what
each member bought is different, which is why this record has two halves rather
than one shared derivation: `check-close-surfaces` removes a spawned derivation
from the gate corpus, and `check-queue-prose-precondition` spends the ERE
matcher on its last held reader.

**The sizing correction the tool does not print, and the general rule behind
it.** The pair is not what its two gate files measure.
`check-close-surfaces.sh`
*spawned* `bin/close-surfaces.sh`, which carried the whole derivation,
so the cohort was roughly 222 shell lines, unevenly split, with
`check-queue-prose-precondition` self-contained over a single-file corpus. The
transferable half is the **cause**: a member's shell-level dependency set is
measured on the **gate file**, so a gate whose work is behind a spawn reads as
*unsized* rather than as large — `check-close-surfaces` sourced only
`lib/gate.sh` while the tool it spawned sourced `lib/stages.sh`. **A spawned
tool is invisible to every static sizing signal the selector has,
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group`'s line counts included.** Stated once here rather
than rediscovered per member. **The class is now exhausted**: the one remaining
unrecorded instance was `check-graph`, whose spawned generator is sized in its
own section (§check-graph) at the 2026-08-17 close that drained the bullet; the
only other spawning member, `check-roadmap-fresh`, always carried its emitter
(76 lines) in `freshness-emitter-port-cohort`'s roster and ported with it.

**A prediction this record corrects, because the oracle overruled it at build.**
The port was expected to make `check-reads-couples`' coverage assertion run *for
real* on this member for the first time — it does not, and the reason is
structural rather than an omission. Both of the derivation's walks hang off a
**computed base** (the scan-root argument, else the toplevel), and `--reads`
declares static strings, so each takes the `?` marker and is skipped-and-counted
exactly as the shell member's unanalyzable walks were. The near miss is worth
recording: declaring the roster tier as `.` filtered by
`LIFECYCLE_KIT_ROSTER_BASENAME` — the filter-knob arm's own shape — *compiles and
reds*, because that spelling claims every tracked `SPEC.md` in the tree including
the docs mirror's, which this member never reads. A declaration wider than the
read is not a safe over-approximation here; it is a false claim the coverage
assertion then holds the gate to.

**The spawn is what the port removes, and the shape it takes is the
freshness-emitter's.** The derivation became a **non-gate arm**
(§The non-gate arm, lifecycle-kit/SPEC.md §The close-surfaces emit arm), the
gate calls it **in process**, and the shell script is deleted. In taking it the
emit-arm class acquired its first member that is **not** a stored projection,
which widened that class's caller clause rather than being smuggled past it. The
in-process call is the point rather than an optimization: it makes
"the derivation and the gate can never disagree" structural, the same dividend
the freshness family banked, and it obliges the descriptor to name both crate
modules under `couples=` or the gate goes untriggered on the edit that breaks it.

**`gsub` is a caller-side loop, and the engine's surface did not move.**
`check-queue-prose-precondition` runs two substitutions over patterns baked into
its own source, and the ERE engine has no substitution arm. The ruling — recorded
with its promotion trigger at §The POSIX ERE matcher — is a twelve-line private
loop over the existing `find`, not a fourth public item with one caller, and not
a bespoke hand-written scanner for the second pattern beside a compiled matcher
that already answers it. `to_ascii_lowercase` and not `to_lowercase`, because a
Unicode fold can change a string's byte length and desynchronize the offsets that
loop slices with.

**Both members landed in one commit, and the rule the landing obeys is about a
member rather than about a commit.** §check-gate-substrate-parity's assertions A
and B are non-monotone in both directions, so *every intermediate state of a
staggered landing is red*: a member's descriptor, module, registry entry and
shell deletion cross together. Two whole members in one commit splits neither, so
it satisfies the rule the same way two commits would; what the rule forbids is
splitting one member across commits.

**The price.** Three crate modules, two descriptors, three shell files deleted,
one renamed SPEC heading with its citations moved, and the generated-projection
fan-out §Generated projections rosters. Parity was proved by running both
substrates over the live tree: the arm is byte-identical to the deleted script's
roster, and the ported precondition gate is byte-identical to the deleted one
over `TASK-QUEUE.md` and over a crafted corpus reaching the bracket-bridging,
past-tense-bridging and multi-byte cases the fixture pair does not.

### What the reverted port established

Not nothing, and worth separating from what it broke:

- Both substrates were **byte-identical** on the fixture pair, on the live tree,
  and on an edge-case tree (uppercase hex, a 39-hex near miss, quoted refs,
  comment leaders, a non-matching dash). The seam carries a gate's rule without
  changing its verdict.
- The manifest survived the substrate change: the generated hook, the graph
  artifact, and every declaration-reading meta-gate kept working against a
  descriptor.
- The conservation table was **exercised rather than imagined** — each
  disposition ran against a real ported member.
- Two relaxations that a port makes tempting were found only because a live
  instance existed: `check-reads-couples` acquired an undocumented
  descriptor-level opt-out (§check-reads-couples), and
  `check-gate-substrate-parity` assertion B turned out to go dark at zero
  descriptors (§check-gate-substrate-parity). Both are fixed; neither would have
  been visible without the port, and neither survives it.

### What is retained, and where the second port stands

Retained: the crate at `native/`, the descriptor spelling and its
specification, the `gate_resolve` / `gate_command` split, the conservation
contract and its disposition table, `check-gate-substrate-parity` with every
assertion that section enumerates, the fixture-runner's substrate-blind
dispatch, the toolchain floor pin, and the CI crate build/clippy/test legs. The
crate carries **both** first-cohort rules — as **live dispatched** implementations
since their descriptors landed, having been `reference-only` while those were held
(§The first cohort, and the rule that selects the next) — so `cargo test` and the
`native_crate` evidence suite assert against real gate rules rather than going
green over an empty crate, and the read-declaration unit tests hold a `?` to its
arity across two members rather than one. Those properties came from the modules
existing, never from their disposition, so going live changed neither.

**The seam was the deliverable; the live port was the demonstration.** A second
port had two prerequisites, not follow-ups. **The `check-reads-couples` half is
satisfied:** the binary-side equivalent exists rather than being opted out of — a
`--reads` arm answering what a member walks, registry-declared and held to executed
behavior by two crate unit tests, consumed by the gate in place of its refusal
(§check-reads-couples, §Meta-gate conservation for the binary substrate). It was
built and proved **without a port**: against the reference-only implementation and
a hermetic fixture, so no `.gate` member had to be added to a kit to end the
refusal. **The criterion 5 half is now satisfied, not merely ruled:** how a
compiled gate arrives in a consumer tree is settled (criterion 5 above), and both
mechanism halves are built — `native-artifact-publish-path` produces the artifacts
and digests, `native-artifact-install-path` selects, verifies and places them.
Neither prerequisite is outstanding, and the second port has since been **built
and proved**: every member of the first cohort ships its rule as a compiled
subcommand, byte-identical to the shell gate it reproduces.

**What stood between that port and an adopter was narrower than it was read to
be, and getting the boundary right is what let the descriptors land.** The first
tag publishing binaries as Release assets was a prerequisite to an adopter
**dispatching** to the binary: a `.gate` descriptor vendors and the binary does
not, so a consumer that *registered* a ported member before that tag held a
member that could not run (§Consumer smoke). It was never a prerequisite to
*declaring*, which is what the cohort's hold was resting on. A vendored
descriptor no consumer registry names dispatches to nothing, and the
load-bearing predicate (§check-gate-binary-fresh) is what makes that distinction
machine-held rather than merely stated. **That tag is cut** — `v0.22.0` — so the
prerequisite is discharged and the distinction is recorded for the boundary it
draws rather than as a live constraint.

**What a landing descriptor *carries* is the freshness oracle — and it was already
in the tree.** A dispatch is what makes the binary load-bearing, and the moment
one is live, `gate_command` dispatching to a prebuilt binary with no rebuild and
no freshness check means a skipped rebuild runs the descriptor-named gate against
a stale implementation. §check-gate-binary-fresh closes that, and it landed
**ahead** of the first descriptor rather than beside it: with nothing dispatching
it reports zero and exits clean, so the commit that finally declared one was
already covered instead of depending on that session rediscovering the hole. The
oracle is now armed rather than dormant, and the arming cost this repo a
commit-time `cargo` (§What the dispatch seam does not settle) — which is the
settlement arriving, not a new prerequisite.

## What the dispatch seam does not settle

Recorded because a deferral nobody wrote down is indistinguishable from a
question nobody asked — and this entry has already lost worked arguments to
compression once.

**Dogfooding is ruled, not deferred: this repo runs built artifacts.** It is
recorded here rather than left to the first live port because making a compiled
toolchain a precondition of this repo's own battery is a change to what the
contract *means*, and a contract change settled by implementation is settled in
the wrong place. Both
halves hold at once — a port is what *determines* the answer, and the answer is
written down before the port lands.

**The mechanism, stated as a rule rather than as a count.** A registered member
dispatching to a compiled subcommand at `tier=precommit` puts `gate_command` on
the pre-commit path, and `gate_command` is fail-closed on an absent binary
(§lib/gate.sh) — so wherever a `.gate` descriptor is live in this tree, the repo
builds and runs the binary at commit time and `cargo` is a **commit-time**
requirement here rather than a build-and-CI one. That is true before and after
any particular descriptor lands, which is why it is a condition on descriptors
and not a claim about how many exist.

**Where the tree stands.** The rule above has bitten: every first-cohort member
is registered and resolves to a `.gate` descriptor, so **gates in `gates.list`
dispatch to the binary and `cargo` is required to commit here.** The toolchain floor pin
(context-kit/SPEC.md §bin/env-probe) has therefore sharpened from its
build-and-CI tier to the **commit-time** tier, and a fresh clone cannot commit
until it has run `bash gate-sdk/bin/build-native.sh` once —
which `cargo test` does not do (§check-gate-binary-fresh). CI still builds,
clippies and tests the crate, and `check-gate-substrate-parity` still reads the
binary's `--list` roster whenever it is built; what changed is that those are no
longer the *only* things that need it.

**The trade, in both directions**, recorded because a ruling whose cost is not
written down gets reopened by whoever first pays it. It **costs**: the battery
depends on a compiled toolchain at every commit, `cargo` moves from a
build-and-CI floor to a commit-time one for this tree, and a fresh clone cannot
commit until it has built the crate once. It **buys** what the alternative
forfeits: running the shell scripts here would keep that toolchain optional and
keep the port's headline benefit unmeasurable in the one tree that exercises it
daily.

**And the alternative is not actually available, which is the ground that stops
this being re-argued as a preference.** A descriptor and its shell script cannot
coexist in one resolve dir (§check-gate-substrate-parity assertion A), and the
kit tree this repo runs its gates from *is* the tree that vendors to consumers.
There is no arrangement in which a consumer dispatches to the binary while this
repo runs the shell script. Ruling *no* would therefore not be "this repo
dogfoods differently" — it would be refusing the port outright, against the
PRIORITY DIRECTIVE (TRAJECTORY.md). The ruling is forced by mechanism, which is
the durable form of the older reading that *porting one gate decides it*.

**Vendoring has left this section — it is ruled, not deferred.** The slice's
claim that it "ships no artifact and changes nothing about how a kit installs"
was false in its second half, and shipping no artifact is what made it false:
the descriptor vendors and the binary does not, so the slice changed how a kit
installs by making one kit's vendored form unrunnable. That is the correction
the revert paid for, and the model that answers it is criterion 5's own text
(§Porting a gate to the binary substrate) — a prebuilt per-target binary in the
payload, digest-verified before it is written, omit-and-declare where the target
roster carries no artifact for the host. The payload half of the claim stands
unchanged: keeping the manifest as tracked text means hook generation runs
consumer-side, so the seam works identically whichever way the payload question
rules.

**The extensibility model has left this section too.** The slice recorded it as
"genuinely unchanged", which with no reason stated is indistinguishable from
unexamined — and this entry has lost two of its three options to exactly that
once already. §The extensibility model rules it.

**Opacity has left this section — it is ruled, not deferred.** The slice was
argued under the reading that opacity is *not* claimed, and that reading is void:
§Consumer payload rules withholding a gate's predicate a goal, and states
precisely what the claim is and is not. The narrower question the opacity ruling
does not reach — whether *this* repo runs built artifacts rather than source —
was sharpened by the pivot rather than answered by it, and is now ruled in its
own right at the head of this section.

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

## Consumer payload

What a gate on the binary substrate **discloses** to the consumer it judges.
Its reach is exactly that: it rules what a *gate* ships, not whether this repo
runs built artifacts — that is the same lever from the other end, and it is
ruled at §What the dispatch seam does not settle. How a compiled gate *arrives*
is ruled at §Porting a
gate to the binary substrate (criterion 5) and implemented by
`native-artifact-publish-path` and `native-artifact-install-path`; this section
is what arrives with it.

**The payload withholds the predicate.** A gate on the binary substrate reaches
a consumer as its `.gate` descriptor, its `# spec:` pointer and the SPEC section
that pointer binds to, its `good/`+`bad/` fixture pair, and a prebuilt,
digest-verified binary. **Its implementation source does not ship.** A consumer
receives everything needed to run a gate, act on its verdict, and verify it
behaves as specified, and does not receive the rule's text.

<!-- payload-discloses: predicate-withheld -->

That marker is the machine tier of the rule the paragraph above states, and this
section is its one owner. canon-kit/SPEC.md §check-payload-claim owns what binds
the two — a governed doc asserting a different disclosure class is a red rather
than a discrepancy a reader has to notice.

This serves the objective TRAJECTORY.md §The objectives records in full — that
opacity is a goal and not a side effect,
because withholding a gate's implementation favours *execution* of it over
*analysis* of it by the coding agents the gate exists to hold. It reverses the
ground the dispatch seam was built under, where opacity was explicitly not
claimed; the reversal is a ruled trajectory pivot, and the earlier reading is
void wherever it survives.

**What the objective buys, stated precisely so it is not oversold.** The
beneficiary is one failure mode: a coding agent told to make a battery green
reads the gate blocking it and edits its way around the predicate instead of
fixing the defect. Withholding the source removes the cheapest path to that. It
does **not** make the rule secret — a binary is reverse-engineerable, the
fixture pair discloses shape, and the SPEC section states the invariant on
purpose. The claim is *raised cost of analysis relative to execution*, never
confidentiality, and no governed surface may state it as the latter. That bound
is what makes the claim honest; it is not a hedge to be softened later.

**What opacity does not extend to, and why each exclusion is load-bearing.** A
withholding rule with no stated boundary grows until the tool is unusable. Four
things ship, each because withholding it would break something the product
needs:

- **The `.gate` descriptor**, because its manifest readers must work with no
  build and no execution — `installer/lib/init.sh` runs `gen-pre-commit.sh
  --write` in the consumer tree (§The `# graph:` manifest).
- **The `# spec:` pointer and its SPEC section**, because a gate that goes red
  without an explicable invariant is an unactionable block, and an unactionable
  block is how a blocking gate turns into a bypassed one.
- **The gate's own output and help text**, because the remedy line is the
  product. A gate that says only *no* is worse than no gate.
- **The `good/`+`bad/` fixture pair**, which is shipping-side and is the
  consumer's whole verification oracle once the source is withheld
  (§Fixture-pair discipline owns what it is evidence of). **Because the pair
  ships, a fixture is payload content**, and payload content is bound by what
  the payload's transport can carry: a fixture that cannot be vendored onto a
  supported host is a **broken fixture** however well it proves its arm. That
  clause is the general rule, and the tracked dangling symlink `check-tree-terms`
  once carried in `good/tree/` is its instance — `tar` cannot create a dangling
  link on a native Windows host, so the packed kit aborted part-way through and
  the arm moved into the bespoke `.test.sh`, which ships too and constructs the
  link at run time (§check-tree-terms).

**The target roster is the surface that asserts platform support.** One Rust
target triple per live line in the file `GATE_SDK_NATIVE_TARGETS_FILE` names
(§Layout and configuration), with one owner and three readers: the publish
workflow's roster job derives its build matrix from it, `scripts/pack-installer.sh`
packs one artifact directory per line and copies the roster verbatim into the
payload as its one publication, and the installer reads that payload copy to
select the host's artifact. A hand-maintained platform list inside the workflow
would be the maintained roster derivation-first refuses, and would leave the
build's idea of the supported set and the installer's idea in two files to drift
silently; **a target in the roster that no build leg produced fails the release**,
which is the correct place for that failure.

The installer needs the roster rather than inferring support from a directory's
presence because the two cases it must separate look identical without it:
*this platform was never committed to* (omit and declare — a supported outcome)
and *this platform was committed to and the artifact is missing* (a broken
payload — a refusal). Collapsing them silently degrades a supported platform into
a green battery over a smaller roster. A payload assembled with no artifacts at
all carries no `artifact/` directory and no roster copy, so it reads as the first
case and never as a payload whose every target went missing.

**A roster line is a support commitment, so it is bounded twice.** It may not
exceed what the project's own install documentation already states, and a target
joins only when a green run has produced and exercised its artifact — not when a
platform is reasoned about, and not when a provider merely offers a runner for
it. A build leg written and never run discharges nothing: *produced and
exercised* is a fact about a run that happened, which is why the bound survives
contact with a plan that looks certain.

**Removing a blocker is not the granting of a permission, and the bound is
unchanged by an unblocking.** Work that makes a target *possible* — the crate
compiling for it, an installer host-map answering for it, an artifact name
deriving correctly for it — moves that target from **impossible** to **eligible
to be measured**, and measures nothing. The join bound above is untouched by
every such change: the target still joins only on a run that produced and
exercised its artifact. This is stated because the failure mode is specific and
likely — an iteration named *unblock* invites its next reader to read the
removal of a blocker as the arrival of a permission, and a roster widened on that
reading is widened on a plan. `x86_64-pc-windows-msvc` is the worked instance:
the crate compiles for it (§build-native, and the cross-check step in the `gates`
workflow that holds it there), and it is **not** on the roster.

**Widening is cheap on the publish path and not free elsewhere**, and both
halves are stated because the cheap one alone reads as the whole cost and is
not. Cheap: the build matrix is roster-derived, so a new platform is one roster
line plus one runner mapping, never a workflow rewrite. Not free: a consumer
smoke that builds its artifact from the host it runs on cannot satisfy a roster
naming a platform that host is not, so the second roster line blocks such a
smoke until it is steered at a narrowed roster through
`GATE_SDK_NATIVE_TARGETS_FILE` (§Layout and configuration) or given a
cross-compiling build. installer/README.md §The consumer smoke owns that
re-entry and records which of the two is built.

**One payload carries every declared target, not one payload per target**, and
that is exactly why the packed **artifact name is derived per roster line from
that line's own target** — `gate_exe_suffix "$target"` appended to the stripped
binary basename, inside the roster loop rather than once before it
(§lib/gate.sh). A single host-derived name is correct only while every roster
line is the host's platform class, which is a landmine that fires on the commit
widening the roster and nowhere before it; it is fixed here, with the roster
still one line, for that reason.

**`pack_tracked` is fail-closed on content it cannot vendor.** The tracked-set
copy is `git archive <commit> -- <root> | tar -x`, so the packed kit is only as
portable as `tar`'s ability to reproduce it, and a **tracked symlink** is the
shape that fails: on a native Windows host `tar` refuses a dangling link — the
kind, file or directory, is picked from the target, and an absent target has no
kind — and the pipeline's status is `tar`'s, so the run reports failure *after*
writing a partial kit. The helper therefore pre-flights `git ls-files -s` for
mode `120000` over the root it is about to pack and **exits 2** naming each
offending path, the platform class it breaks and the remedy, before writing
anything. Fail-closed at the producer is the whole enforcement and it is exact:
it covers precisely what is packed — every kit root plus `installer/`, both going
through this one helper — with no corpus knob to configure and no roster to
maintain, and it is exercised on every path that assembles a payload, the release
pack included. A refusal that writes nothing and names its cause beats a failure
that half-vendors, and the difference matters most on the host that cannot debug
it.

The artifacts are never produced from a working
tree: the pack step takes them
from the run artifacts the build legs uploaded and builds nothing itself, so a
locally-built binary can never substitute for a released one — the same reasoning
the vendoring ruling applied to the crate source, arriving one layer out. **The
consumer smoke's host-built artifact is a harness stand-in, not this rule
relaxing**: it builds from a working tree because it has no Release to draw on,
and hands `pack-installer.sh` a directory it did not produce, exactly as a build
leg would (installer/README.md §The consumer smoke). The
one-payload shape is ruled on the numbers: the *installed* footprint is one
binary either way, since the installer writes only the matching target, so the
difference against a per-target payload is download size alone and is bounded by
the roster — while the per-target shape multiplies the publish path, the digest
set and the attestation surface by the roster size. **The revisit trigger is a
measurement rather than a taste:** the roster growing past what a CI matrix
commits to, or one target's binary ceasing to be small.

**The obligation opacity buys.** A consumer who cannot read the gate has only
the publisher's word for it, so the integrity story is the whole of what
replaced reading the source. The achievable floor is a published per-target
digest verified before the artifact is written (`native-artifact-install-path`).
Each build leg writes `<binary>.sha256` beside its binary, in `sha256sum -c`
format with a bare filename inside it — the same shape the release already uses
for the tarball, and the shape the install page documents a reader verifying.
**One producer, two publications, and no recomputation anywhere:** the bytes the
build leg wrote are the bytes the payload carries *and* the bytes the Release
attaches, because every later hop moves the file rather than re-deriving its
contents. A second `sha256sum` on a later job is exactly what lets a published
digest and an installed digest diverge while both look computed, so the rule is
held mechanically by §check-gate-substrate-parity assertion F rather than by
review. Per-artifact sidecars rather than a combined `SHA256SUMS` or a JSON
manifest, because an attestation's subject list is `{name, digest}` pairs: a
sidecar maps onto a subject one-to-one and `tarball-build-attestation` can later
land *beside* these files with no migration and no digest value changing, where a
manifest would mint a schema and a version key for the same information.

The Release publishes the per-target binaries and their sidecars alongside the
tarball, renamed per target because Release assets are flat. The **content** of
each sidecar is left alone — its bare filename is the name the file carries in
the payload, where the machine verification happens, and rewriting it would mint
a second spelling of one published fact. What that publication buys is that the
digest an installer verifies against has a source outside the payload it travels
in; a digest shipped only alongside its own artifact certifies nothing.

**What the digest proves, at its honest bound.** It is a transfer- and
substitution-integrity claim: the artifact in the payload is byte-identical to
the one the release built. It is **not** evidence the build host was
uncompromised.
What that floor does not provide is a reproducible build, and the queue holds
that ground as `tarball-build-attestation`: the checksum proves transfer only.
The pivot changes what that entry is worth rather than what it says — while
sources shipped it was a supply-chain nicety, and with sources withheld it is
the consumer's only remaining basis for trust.

**The rule is held by structure, not by discipline.** It is violated
structurally — a ported gate's implementation source reaching the vendoring set,
which is exactly the kit roots — so it is checked structurally, by
§check-gate-substrate-parity assertion E. The gate a prior reading of this
question proposed, a NUL-byte scan enforcing *no artifact ships*, is dropped
with its reason on record rather than carried on momentum: it enforced the
ruling under which shipping a binary was the violation, and under this one
shipping a binary is the point.

## The extensibility model

How a consumer extends the battery, ruled rather than deferred. Three shapes
were ever on the table and two of them were dropped once with no rejection
recorded, which is how a later session re-derives a settled question from
nothing.

**The shell escape hatch stays first-class.** A consumer authors their own gates
as shell in their own resolve dir; resolution stays consumer-first and `.sh`
beats `.gate` within a dir (§lib/gate.sh), so a consumer shadowing a ported kit
gate with their own script still wins. Withholding a gate's predicate does not
narrow this, because it is a choice made on the consumer's own machine about the
consumer's own code — it puts no interpreter into the *shipped payload's*
dependency floor, which is the floor the trajectory governs. Forbidding it would
strand every consumer-authored gate to buy nothing.

**A declarative check DSL is refused.** A DSL is a language carrying none of a
language's tooling, and this repo's own battery is the evidence against its
expressible set: the gates carrying real judgment are exactly the ones a rule
language would not hold. The declarative half a DSL is wanted for already exists
as the `# graph:` manifest, which every reader greps as text with no build
(§The `# graph:` manifest).

**Native plugins are refused**, and §Consumer payload strengthens that refusal
rather than weakening it. A dynamically loaded third-party plugin is a stability
contract this project would own forever *and* an unattested execution path
inside the one artifact whose integrity the pre-write digest verification exists
to guarantee. Opacity any loaded object can step around is not opacity. The
neutral authoring surface a plugin ABI would foreclose is
`gate-authoring-sdk-surface`'s to design, and refusing the ABI is what keeps it
open.

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

**This harness does not run the installer, and that is what bounds what it
proves.** It vendors kit roots **by copy**, so no payload, no digest and no
`# omitted:` record are in play: its scratch consumer registers whatever each
`smoke/install.sh` registers, and it asserts the **kit defaults** under **zero
consumer config**. Proving the *install* path is the installer's own smoke
(installer/README.md §The consumer smoke), the one caller that packs a payload
and runs `init` against it. The two harnesses must not blur: this one answers
*do the kits work when vendored*, that one answers *does the installer deliver
them*.

**Its default run vendors every kit, so its silence on a subset vendoring is not
coverage.** `run-consumer-smoke.sh` takes kit roots as arguments, and the
subset invocation — vendoring one kit while the shared binary carries every
ported kit's subcommands — is a real configuration this harness can be put in and
is not put in by any scheduled run. That configuration is covered instead by the
bespoke `check-gate-substrate-parity` test, which reaches it in a sandbox at
commit time (§check-gate-substrate-parity); recorded here so a later reader does
not read the default run's green as an answer about the subset.

**It does place the gate binary, and the rule is stated rather than left as an
exception, because a reader will otherwise lean on the older
no-binary-by-design sentence and find it gone.** The rule: **a vendored member
that dispatches must be able to run, so the scratch consumer receives an
already-built artifact out of a checkout the caller names**
(`csmoke_place_binary`, §lib/consumer-smoke.sh).
The ground is that a kit root vendoring a `.gate` descriptor is now the ordinary
case — the first cohort ships two (§The first cohort, and the rule that selects
the next) — and a kit's `smoke/install.sh` may legitimately register a ported
member, which `site-kit`'s does: it copies `templates/site-health.yml` in, and
that template is the only Actions-shaped surface any install writes, so
§check-action-pinning and §check-action-gh-repo earn their scratch-battery slot
under *The registration accounting* below. Refusing the binary would have made
that registration permanently red and forced the coverage out, which is the
opposite of what this harness exists for.

**Whose artifact it is, is the caller's to name.** The checkout whose `native/`
was built is the function's first argument. It was once resolved from the
library's own `BASH_SOURCE`, and a shared library deciding from its own location
whose artifact a caller receives is a defect that stays invisible until some
caller wants a different one. `upgrade-smoke` is that caller: it runs two refs'
vendored shell against one scratch consumer, and under the old resolution both got
the *invoking* tree's binary — so FROM's shell ran against TO's binary and the
mismatch was reported as a broken tag (§upgrade-smoke). `run-consumer-smoke.sh`
and context-kit's `smoke/agents-md.sh` pass the invoking repo and behave exactly
as they did. Whether a binary is wanted at all is `csmoke_gate_descriptors`,
factored out of the placement so a caller that must *produce* one can ask the same
question a step earlier rather than keep a second copy of the predicate.

Three constraints keep the placement from becoming a second install path. It is
a **copy, never a build** — the artifact comes from `GATE_SDK_NATIVE_BIN` in the
named checkout, which for the in-repo callers is already obliged to be current
(§check-gate-binary-fresh), so this harness acquires no toolchain of its own.
That a caller may have built the checkout it names does not change the rule here:
`upgrade-smoke` builds one binary per ref before calling, and that requirement is
its own and stated there (§upgrade-smoke). It
lands at the **kit default** path and writes **no consumer config**, deliberately
unlike `init`, which places the artifact in the gates dir and points the knob at
it — that is the install path and it belongs to the other harness. And it is
**derived**: the vendored kit roots are scanned for `.gate` descriptors, so a kit
set with no ported gate gets no binary and needs none, while descriptors present
with no built artifact is an environment failure (exit 2) rather than a red
battery. The placed artifact is **gitignored** in the scratch tree, because the
violation phase restores with `git clean -fd`, which spares ignored paths and
would otherwise delete it between kits.

**One registered meta-gate is deliberately not in the scratch battery.**
`check-gate-substrate-parity` is registered — it needs no consumer config and a
vendored tree is exactly where a descriptor with nothing behind it would land.
`check-gate-binary-fresh` is **not**, and the ground is the one §check-gate-binary-fresh
states: *a consumer tree does not verify a build it did not make*. Its subject is
the crate the binary was built from, §Consumer payload keeps that crate outside
every kit root, and no install writes one — so the gate's subject does not exist
in a vendored tree at all. It carries **no `# smoke-unregistered:` declaration**,
and the absence is the contract rather than an oversight: the probe below derives
the exemption every run (exit 2 in the scratch consumer, exit 0 in the invoking
repo, the surface genuinely absent), and a written reason where the probe already
exempts is the inversion *The declaration valve* names as a finding. Recorded
here because a reader finding an omission with no reason beside it will otherwise
add one and redden the harness.

**What a ported gate a `smoke/install.sh` does *not* register looks like.** Its
descriptor vendors with the kit root and nothing dispatches to it, so both binary
meta-gates read the live registry and report clean, naming the descriptor count
beside the dispatching count (§check-gate-binary-fresh). A member a
`smoke/install.sh` *does* register while no binary exists still reds, through
`gate_command`'s harness error (§lib/gate.sh) — that path is intact, which is
why the placement above exists rather than a relaxation.

The scratch-consumer build itself — temp dir, seed commit, vendor-by-copy, the
binary placement, the `smoke/install.sh` loop, the installed-baseline commit — is
factored into `lib/consumer-smoke.sh` (`csmoke_vendor_and_install`, which sets
`SCRATCH` and `CSMOKE_INSTALLED`, and `csmoke_place_binary`, which it calls), so a
second harness that needs the same green baseline before it diverges shares the
mechanics rather than copying them. The placement lives in the shared builder
rather than in `run-consumer-smoke.sh` for the reason the factoring exists at
all: every caller builds a tree the vendored kits' descriptors are in, so a
placement in one caller would leave the others reddening identically. The caller
owns its cleanup trap and every assertion after the baseline commit.
context-kit's `smoke/agents-md.sh` is that second caller: it builds the same
baseline, then converts the consumer to a nondefault agent file (`AGENTS.md`)
and asserts the agent-file knobs carry it — an assertion `run-consumer-smoke.sh`
cannot make, since it fixes the kit defaults under zero config
(context-kit/SPEC.md §Testing). `bin/upgrade-smoke.sh` is the third caller: it
builds the same FROM baseline, then diverges into the two-phase upgrade proof
(§upgrade-smoke).

**The library's sourcer set is wider than the builder's caller set, and the two
are counted separately.** The three named above are the callers of
`csmoke_vendor_and_install`. The file itself has a **fourth** sourcer:
`demo/run-demo.sh`, which sources it for `csmoke_place_binary` alone and never
builds a scratch consumer at all. So "sourced by three" is false of the library
and true only of the builder — read the distinction off this paragraph rather
than counting callers of one function and generalising, which is the
re-derivation that put a wrong count into an amendment.

**The `smoke/` per-kit contract.** Every vendored kit ships a `smoke/`
directory — shipping it joins fixtures + README + SPEC in the kit-landing
checklist; a kit root lacking `smoke/` is an environment error (exit 2). Every
`smoke/` script that mutates the invoking tree — `install.sh` and
`violation.sh` both do — opens with the entry-point guard
`: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"` before its first mutating
command, so a bare invocation (outside the harness that exports
`SMOKE_KIT_ROOT`) refuses instead of writing into the caller's repo;
`check-smoke-entry-guard` (§check-smoke-entry-guard) holds the guard's presence
across the roster. Every script in that roster stays on the shell substrate
permanently and declares `# no-port:` saying so — the class ruling, its four
legs and what would reopen it are *The port disposition* below. The
README item of that checklist carries the register-the-gates block in
`<!-- gate-roster:begin -->` / `<!-- gate-roster:end -->` markers, held in
name-set parity with the kit's shipped `checks/` by `check-readme-roster`
(§check-readme-roster) — a kit that ships checks registers them. Every gate in
that roster carries its `# install:` declaration, one more clause of the same
checklist and the one `check-install-disposition` holds (§The install
disposition). A new gate MAY
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
  to be green, and regenerate the hook + graph artifacts. **Copying the kit's own
  templates in is not part of that contract**: a smoke script installs a template
  only where a registered gate reads one — site-kit's `templates/site-health.yml`
  is the shipped instance — so a kit whose templates no gate reads copies none,
  and whether a given kit copies is read off its script rather than assumed.
  *Which* gates it
  registers is not the author's discretion — *The registration accounting*
  below rules on every omission, and the installer's own narrower subset is
  derived from the same per-gate declarations (§The install disposition) rather
  than listed anywhere. This roster is the **superset** of that subset, held so
  by `check-install-disposition` assertion B. It may assume gate-sdk
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

**Both declaration spellings, and the probe dispatches rather than executing a
path.** The union is over `check-*.sh` **and** `check-*.gate`, and each probe
resolves its invocation through `gate_command` (§lib/gate.sh) instead of running
`<dir>/<name>.sh`. Stated because the shell-only reading was live and silent: a
gate whose implementation ported left this pass's universe entirely — not
probed, not declared, not reported — so the one mechanism that catches an
unregistered gate stopped seeing it at exactly the moment its substrate changed.
That is the same silent-departure defect §Meta-gate conservation for the binary
substrate exists to prevent, arriving through a `bin/` tool that assertion C's
runtime derivation cannot reach because it is not a registry member. A ported
gate earns or forfeits a scratch-battery slot on the same terms as any other.

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

**A declaration the probe contradicts is reported, not silently exempted.** A
gate declaring `zero-config` (§The install disposition) asserts it reads a
surface a fresh install writes; a probe finding that same gate surface-absent in
the scratch consumer has derived the opposite. That is a **contradiction between
a declaration and a derivation**, and the accounting names it as one rather than
granting the ordinary self-declared exemption — which is the arm that keeps a
*wrong* declaration from being as invisible as a missing one, the missing one
already being `check-install-disposition` assertion A's. The verdict is
informational here rather than fatal, because the scratch consumer is not the
tree `init` makes and a `zero-config` gate is legitimately unregistered in this
harness's roster; what it must never be is unremarked.

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
doc. What that question settled instead is §The install disposition — the
kit-owned fact is the per-gate **disposition**, and the roster the *installer*
starts a consumer with is derived from it, while this script stays the
executable recipe for the smoke's own richer tree. The disposition is not added
to the README block either: that block is prose an adopter pastes, and a
machine-read annotation inside a pasted snippet is a second grammar in a surface
that has one.

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

### The port disposition

**Every `smoke/` install and violation recipe, and both members of this harness,
carry `# no-port:` — ruled 2026-08-30 for every kit `gate_kit_roots` vendors, on
four measured legs and none of them size.** The declaration sits on
`bin/run-consumer-smoke.sh` and `lib/consumer-smoke.sh` here, and on each kit
root's `smoke/install.sh` and `smoke/violation.sh`; §port-blockers' `--tree` arm
reclassifies each one `owed` → `no-port`, so the completion predicate
TRAJECTORY.md states over that arm's owed count subtracts them. **It is not a
size judgment and must not be read as one**: the criteria relaxation is closed at
§The port-candidate criteria as an ordering signal, never an eligibility screen,
so what decides this class is what its members *are*. **The cut was re-put to the
operator with the measurement that it resolves to declarations and writes no
Rust, and it stands** — a cut is not less legitimate for resolving to
declarations when the declarations are structural, and re-cutting for Rust was
the alternative refused.

**Leg 1 — the config bridge, and it is the load-bearing one.** *The registration
accounting* above probes each unregistered gate through `gate_command`
(§lib/gate.sh), which builds the bridge by sourcing each owning kit's `lib/*.sh`.
§lib/gate.sh rules **exactly one place a knob's value is computed**, so a
crate-side probe would be the second producer criterion 6 refuses. Nor can the
probe be delegated back to a bash front-end: `run-gates.sh --only` resolves
against the registry, and the accounting's entire subject is gates that are
**not** in it — which is why this harness calls `gate_command` directly rather
than the front-end. That is the same structural ground §gen-pre-commit is
already declared on, reached from the opposite direction: the hook generator
bakes a resolved knob, this harness resolves a knob for a member no registry
names. The two harness members rest here.

**Leg 2 — a `smoke/install.sh` is an executable recipe by stated contract, and
porting one is non-monotone for a live reader.** *Not derived from the README
roster* above already declined to derive the registration out of a doc, "on a
boundary, not on merit: it would turn `smoke/install.sh` from an executable
install recipe into a derivation over a doc" — and a crate table is that same
boundary crossed harder. The script is also read **as text** by an oracle:
§check-install-disposition assertion B reds on a kit that ships a `zero-config`
gate and no `smoke/install.sh`, and its registration arm greps the script body
for each gate name. Deleting or de-textualizing these scripts therefore *adds*
violations rather than removing them.

**Leg 3 — the class costs an adopter no interpreter dependency, which is the
objective the port serves.** A `smoke/` directory is kit-authored content
(§Consumer payload's kit-authored roster names it) and vendors to an adopter with
its kit — conceded rather than disputed — but it is **executed by no adopter
path**: the entry-point guard above refuses a bare invocation, and the only
callers in existence are this repo's own validate suites. It ships inert.

**Leg 4 — the envelope, measured class-wide rather than argued.** Measured at the
2026-08-30 cut, roughly three quarters of the in-contract `smoke/install.sh` line
count sat in the four files that drive and assert on *their own kit's* `bin/`
tools — lifecycle-kit's, gate-sdk's, doctrine-kit's and evidence-kit's — against
tools owned by seven **other** stated contracts, every one of them still owed.
Porting those four moves a behavioural envelope this contract does not own, which
is the hazard the drift-kit cut's held member showed on one file and this
measurement shows across the class. The rest are pure recipe and fall to legs 2
and 3. What rides on getting this wrong is stated with the leg rather than left
implicit: a kit's `smoke/` is the only end-to-end behavioural oracle it has — a
fixture pair proves one gate in isolation, this proves the kit installs and runs
under zero config — so the class is not moved casually whatever a size arm says.

**The ruling reaches by ground, not by scope.** A stated-contract cut reaches
only the files answering to this section, and the class is wider than that: the
`smoke/` recipes under context-kit/SPEC.md §Testing, delegation-kit/SPEC.md
§Testing and drift-kit/SPEC.md §Testing are the same shape and take the same
disposition because legs 2 and 3 hold of them, not because this section reaches
them. Each declares in its own header and its own SPEC section says so.
`context-kit/smoke/agents-md.sh` is **not** a member — it is a validate-suite
driver rather than an install or violation recipe, and it stays owed.

**The honest limit, stated because this ruling does not answer it.** The smoke
corpus is a worked, cleartext catalogue of exactly what reddens each gate, and it
vendors to every adopter. That is the analysis surface §Consumer payload's
opacity ruling wants raised, and it is the one live argument for porting this
class. This disposition leaves that argument standing rather than refuting it: a
session that wants to answer it is answering a §Consumer payload question, not a
§Consumer smoke one.

**A standing ruling names this exact corpus, and it is reconciled here rather
than reversed.** TRAJECTORY.md §The closed rulings, 2026-08-28 corrected the
2026-08-23 carve-out in place to read that kit `smoke/` suites and kit-resident
test runners ride the installer payload with their kit roots and land committed
in adopter trees, so they are *kit mechanism on the claim like any owed file*,
the residue genuinely shipping to no adopter taking a per-file disposition when
reached; two paragraphs later it refuses "a contributor-side `# no-port:` class"
over that same corpus, on two counts. Both are live objections to this
disposition and both are answered on the text:

- **The "when reached" clause is not scoped to the three named no-adopter
  examples.** The ruling's own construing sentence restates over "each file [the
  carve-out] covers", and the carve-out it construes covers contributor-side
  tooling and the test harness as a whole, where kit `smoke/` suites sit. The
  three-name list belongs to the earlier sentence being corrected, not to that
  restatement. The "case-by-case residue rule" it names is the 2026-08-09
  PRIORITY DIRECTIVE — "surviving shell is residue justified case by case, never
  a protected category" — and four measured legs, each cited on its own file, is
  that reasoning applied to a class whose members happen to share legs, rather
  than the categorical "it is a smoke harness" exemption the directive refuses.
- **The refused ground is not this ground.** "Ships to no adopter" was measured
  false for 31 of the 33 disputed files, and leg 3 **concedes** that measurement
  rather than disputing it — vendoring with the kit is leg 3's own opening
  clause. What leg 3 rests on is narrower and untouched by it: *executed by no
  adopter path*. Legs 1, 2 and 4 do not use the shipping question at all. The
  2026-08-28 ruling closes by saying no mechanism is missing, "only a standing
  ruling that would make one true, and none does"; four measured,
  individually-cited structural grounds are that ruling now existing.

**The counterweight is real and it was ruled against, not absent.** The
2026-08-28 text's contrast — kit `smoke/` "on the claim like any owed file"
against a residue that "takes a per-file disposition" — genuinely reads, on its
own, as putting this corpus on the must-port side, and a seventeen-file sweep is
class-shaped even when every line cites its own ground. It is recorded here in
those terms so a later reader meeting that passage alone lands on this ruling
rather than re-opening the question it already answers.

**What reopens it**, written as a reopening condition rather than a permanence
claim, because `# no-port:` is the permanent tier and a class ruling owes its
reader what would falsify it: leg 1 dissolves if §lib/gate.sh ever admits a
second bridge producer; legs 2 and 4 dissolve together if
§check-install-disposition assertion B stops reading the script as text **and**
the `bin/` tools the four envelope files drive are themselves ported. Leg 3
dissolves if any adopter path executes a `smoke/` script.

## Per-component contracts

### lib/gate.sh

The family's single sourced library — values + adapters, never gate structure.
It gives a gate author the fail-closed guard `fail_closed`, the walk adapters
(`gate_find` / `GATE_GREP_EXCLUDES` / `gate_path_pruned` over the dirs
`GATE_SDK_PRUNE_DIRS` names, plus whatever `GATE_SDK_PRUNE_EXTRA_DIRS` appends),
the registry helpers that resolve a check consumer-first across kit dirs
(`gate_resolve`, `gate_kit_roots` / `gate_kit_roots_rel`, `gate_check_dirs` —
the multi-kit resolution path other kits' gates ride), and the `# graph:`
manifest readers (`gate_expand_couples` and its siblings §The `# graph:`
manifest). How each derives its result lives in the source; the invariants a
reader needs outlive the refactor that renames a helper:

- **The prune set is one array, read by all three walk adapters**, which is why a
  member added to it needs no per-gate edit. Its members are directories that are
  never a consumer's source: third-party build and dependency output (`target`,
  `node_modules`), git's own store (`.git`), this methodology's scratch (`.tmp`),
  fixture corpora a gate must not judge as tree content (`gate-tests`), and
  `worktrees` — the leaf under which an agent harness materializes a **second full
  copy of the repo**. That copy is the reason the member is a kit default rather
  than consumer config: every tree-walking gate descends into it, so a live
  isolated dispatch reddens the battery with findings that name the session's own
  files, and every consumer running isolated dispatch hits it identically. The
  match is on the **leaf basename**, so `worktrees` selects that directory and
  nothing else — the parent is not named and its siblings are not taken.
  **The caveat a green battery cannot tell you**: pruning the parent instead also
  passes, because that parent typically holds only markdown and JSON, which the
  shell/Rust walk never collects — but the governed markdown surfaces under it are
  read by *explicit globs* no prune touches, so the blunt variant loses coverage
  silently rather than reddening. Prune the leaf. The same silence is the residual
  risk of the member itself: a consumer whose own source lives under a directory
  named `worktrees` (or `target`, or `.tmp`) loses coverage with no red, and
  reaches for `GATE_SDK_PRUNE_DIRS` to restate the set without it.
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
- `gate_kit_roots_rel` emits the roots relative to **the directory holding the
  kits**, which it derives from `gate_sdk_root` — the library's own location,
  never the caller's working directory and never the git toplevel. That is the
  anchor the couples globs share, and it is cwd-independent by design so a
  `cd`-ing caller still gets stable names. The consequence a caller must hold:
  the *names* come from the library's own tree, so a script that resolves those
  relative roots against some other tree gets one tree's kit set spelled over
  another tree's contents.
- **`GATE_SDK_PROGRAM_FLOOR` is the one home of the payload's assumed-program
  set**, read by §port-blockers at the transition where a command-position word is
  classified as a criterion-7 requirement or discarded. It is a kit default rather
  than a literal in the tool for the reason every other knob is: a consumer
  shipping a different floor — a stripped container, a busybox userland — repoints
  it instead of patching the derivation. `git` is a member because §The
  port-candidate criteria already rules it the sanctioned exception, *"because it
  is the floor"*, and the set is written here rather than restated in the SPEC
  prose that cites it.
- `gate_native_targets` is the **target roster's single reader**: one Rust target
  triple per live line, `#`-comments and blank lines stripped by the same
  `gates_list_members` grammar `scripts/gates.list` uses. An absent roster emits
  nothing and returns 1, so a caller tells *no roster declared* from *a roster
  declaring nothing* rather than reading both as no targets — the distinction
  §Consumer payload's omit-and-declare path turns on. Its two path accessors,
  `gate_native_targets_file` and `gate_native_bin`, exist so each knob default has
  one home across the readers that gained one with the artifact path (the publish
  workflow, `scripts/pack-installer.sh`, §check-gate-substrate-parity).
  `gate_native_crate` is the third, holding `GATE_SDK_NATIVE_CRATE`'s default and
  its trailing-slash stripping in one place now that the knob has three shell
  readers rather than one.
- `gate_exe_suffix [<triple>]` is the **executable suffix's single owner**, and no
  other surface in any kit spells `.exe`. It is the tree's existing
  dialect-dispatching helper, and §The path-dialect contract is what it dispatches
  *on*: the `MINGW*|MSYS*|CYGWIN*|Windows_NT` host match below is that contract's
  two-dialect host, recognised here for the artifact-name question and there for
  the root-spelling one. It prints `.exe` and nothing else, or
  empty. Two forms: given a non-empty **target triple** it answers for that triple
  (`*-windows-*` matches, everything else is empty); given **no argument — or an
  empty one, which *is* the host triple, being the shape a `--target`-less cargo
  build emits for** — it answers for the **host**, matching `uname -s` against
  `MINGW*`, `MSYS*`, `CYGWIN*` and `Windows_NT`. Three readers take it and each
  picks its form from what it is naming: `GATE_SDK_NATIVE_BIN`'s default (§Layout
  and configuration) takes the **host** form, because the knob names a binary on
  the machine resolving it; `bin/build-native.sh`'s `BN_ART` and
  `scripts/pack-installer.sh`'s per-roster-line artifact name take the **target**
  form, because both name an artifact built *for* a triple that need not be the
  host's (§build-native, §Consumer payload). `installer/lib/init.sh`'s
  `select_artifact` deliberately takes **neither**: it discovers the artifact name
  with `find … -maxdepth 1 -type f ! -name '*.sha256'` and asserts exactly one, so
  it is already name-agnostic and a `.exe` satisfies it unchanged — named here
  because the instinct is to add a fourth reader, and adding one would replace a
  working derivation with a spelling. The crate is **not** a kit and holds the
  *other* suffix question — what an already-installed program may be named, from
  `PATHEXT` — under its own single owner (§Fail-closed contract); nothing there
  names a built artifact, and nothing here reads `PATHEXT`.
- `gate_native_source_stamp` is the **tree side of the source stamp**
  (§check-gate-binary-fresh) and the only shell spelling of it: the same three git
  invocations `native/build.rs` bakes into the binary, so the comparison is one
  algorithm rather than two implementations of one. It emits the stamp, and
  returns 1 emitting nothing when git cannot answer, so a caller fails closed
  rather than comparing against an empty string. Any edit here that is not the
  same edit in `build.rs` is the canonicalization drift git-as-hasher exists to
  prevent, which is why a crate unit test holds the two to agreement.

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
  one-element `<dir>/<name>.sh`, or the two-element `<binary> <name>` —
  prefixed, when the member declares knobs, by the config bridge below. Its
  callers are the execution sites: gate-sdk's own — §run-gates, §run-gate-tests,
  and §gen-pre-commit, which does not execute the argv but *emits* it into the
  hook — and, since the lifecycle-kit cohort, **a kit's own `bin/` and `smoke/`**,
  which reach it for the same reason and are the first callers outside the test
  lane. The contract they read is the one already stated here, with one asymmetry
  worth naming where the argv is: a caller that resolves and executes from
  different directories must pin `GATE_SDK_NATIVE_BIN` absolute, because the
  default is repo-relative (§Layout and configuration).
  **It emits two distinguishable failure signals, and they are told apart only by
  its status.** `return 1` means the member resolves in **no** check dir — the
  caller's own diagnosis to name. Any other non-zero status is a **harness
  error** — an absent or non-executable binary for a `.gate`-dispatched member,
  or a config-bridge knob refusal — raised as an `exit` from inside
  `gate_command` after it has already written a naming reason to stderr. The two
  are indistinguishable on stdout: both leave the caller with an **empty argv**.
  So **a caller must capture the status, which means a command substitution and
  never a process substitution**: an `exit` inside `< <(…)` kills only the
  subshell and reaches the caller as emptiness alone, and a caller that models
  emptiness as "resolves in no check dir" then re-diagnoses a build problem as a
  missing gate. §Fail-closed contract states the trap and the required branch for
  the class.

**The array-knob config bridge.** A consumer's kit knobs are bash arrays
resolved by a shell library that is sourced *in-process by each shell gate*,
after it is already the exec'd process. A compiled gate never runs that source,
so without a bridge it sees none of them — and `GATE_SDK_PRUNE_DIRS`, a
whitespace-separated scalar, is the one precedent that does not generalize.
`gate_command` prints argv and every call site captures it in a subshell, so it
cannot export into its caller; the bridge therefore rides the argv it already
emits. For a `.gate` member declaring knobs the emitted argv becomes

```
env
GATE_SDK_KNOB_<NAME>=<tab-joined values>
…
<binary>
<name>
```

A keyed knob takes the same line with `<key>=<value>` in place of each element:

```
GATE_SDK_KNOB_<NAME>=<key>=<value><TAB><key>=<value>…
```

`LIFECYCLE_KIT_PREDECESSOR` is the live instance: it crosses as one
`<stage>=<predecessor>` element per declared pair, sorted by key, whichever pairs
the consumer's own config declares.

one element per line, the existing protocol unchanged. **A member declaring no
knob emits the two-element argv exactly as before**, so the bridge is inert for
a member that reads no config. The declaration is the crate's, not the
descriptor's: the binary answers `--knobs <name>` with one knob name per line
off registry data a member cannot compile without, and `gate_command` asks it
for every `.gate` member before emitting argv. That is the same
*registry-data-held-to-executed-behavior* shape as `--reads`, and it is what
makes an unread bridged knob impossible — the crate declares only the knobs its
own code reads.

Resolution, per declared knob:

- **The owning kit comes from the knob's own `<KIT>_<KNOB>` prefix**, mapped to
  a kit root through `gate_kit_roots`: each root's basename, hyphens to
  underscores and upper-cased, is tried as a `<KIT>_` prefix. A knob matching no
  root's prefix resolves to **gate-sdk itself** — never a parse error and never
  a third kit guessed at. That fallback is not a second rule bolted on: gate-sdk
  is the one kit every `.gate` dispatch already runs inside, so *no other kit
  claimed it* and *it is gate-sdk's own* are the same fact. `GATE_PRUNE_DIRS` is
  exactly that case, and the reason it exists: it is gate-sdk's *resolved*
  library global, deliberately distinct from the `GATE_SDK_`-prefixed consumer
  overrides that feed it, so it carries no `GATE_SDK_` prefix of its own.
  Derivation-first — no roster of knob→kit pairs is maintained, so none can rot.

  **Some knob names are *un-declarable* by construction, and a port that declares
  one fails closed on its first run.** A name defaulted only inline, whose value a
  *resolved sibling* the bridge already carries subsumes, has no top-level
  assignment for `declare -p` to find, so naming it in a member's declared set hits
  the emitter's does-not-define refusal. `GATE_SDK_PRUNE_DIRS` and
  `GATE_SDK_PRUNE_EXTRA_DIRS` are the pair to watch, because prose in this document
  names them together as the source of the prune set: that sentence describes the
  **conceptual** pair, and the bridgeable spelling is the resolved
  `GATE_PRUNE_DIRS` alone. The same shape covers the kit-root override, whose
  bridgeable spellings are `GATE_KIT_ROOTS_HERE` and `GATE_KIT_ROOTS_REL`. The trap
  runs the other way too and is worth stating with it: a knob spelled as an inline
  default at its use site can still be perfectly declarable, because the guarded
  top-level assignment exists elsewhere in this library. **Neither direction is
  visible from a use site**, so the verdict is taken at the library
  (§The sixth budget batch, which surveyed twenty-eight crossing names to find it).

  **A distinct spelling is what a grammar change buys, and nothing else, so a
  gate-sdk knob that changes no grammar keeps its own name.** `GATE_SDK_PRUNE_DIRS`
  is a whitespace-separated scalar feeding an *array*, and one name meaning two
  grammars is the defect the closing paragraph of this subsection names. A
  scalar-in/scalar-out knob has no such collision, so it is resolved in
  `lib/gate.sh` under the consumer's own knob name and bridged under it — the
  shape every `QUEUE_KIT_`-prefixed knob already crosses in. `GATE_SDK_REGISTRY_DOC`
  and `GATE_SDK_RUNNER_DOC` are the live instances: resolved here so the bridge's
  does-not-define refusal cannot fire on them (§check-kit-registration), with the
  prefix rule reaching them through gate-sdk's *own* prefix rather than through
  the no-kit-claimed-it fallback. Both routes land on the same kit, which is why
  either spelling resolves and why the choice is a naming one.

  **The configured set is consulted first, then the shipped one, and the order
  is load-bearing.** `GATE_SDK_KIT_DIRS` narrows which kits a battery *scans*;
  reading it as the set of kits that *exist* leaves a narrowed run unable to
  attribute another kit's knob, so it falls through to gate-sdk and fail-closes
  on every member that declares one. A narrowed run is not a hypothetical — it
  is how the kit-root prune is exercised at all, since a prune only bites on a
  kit root below the scan root. So the lookup tries `gate_kit_roots`, then the
  shipped set the `checks/`-or-`smoke/` predicate derives, then gate-sdk.

  **Those candidates are read to EOF before the match loop runs**, never
  streamed through a `while read` the first prefix hit returns out of. The
  lookup runs under a stdout capture, so an early hit that abandons the producer
  leaves it writing into a closed pipe; §run-gates gives the consequence, and
  why the environment that shows it is not the one a battery is usually run in.
- **That subshell inherits the caller's environment, so a bridged member's knob
  resolution can be aimed at a config the caller chooses — which is the
  affirmative form of the property below rather than a second one.** The
  resolution sources the owning kit's `lib/*.sh` inside a subshell of the
  dispatcher, and a kit library resolves its `<KIT>_CONFIG_FILE` from that
  environment, so a harness (a bespoke scenario runner, a fixture driver) that
  exports `<KIT>_CONFIG_FILE` before calling `gate_command` points a **compiled**
  member's knobs at a synthetic config. The answer a port needs is therefore
  yes: a ported member still takes a config file, through the knob rather than
  through a positional. §lib/test-hermetic.sh uses exactly this to pin every kit
  at one empty file; that is the pin-to-empty case, and the general affordance
  is stated here so it is not re-derived off the bridge's implementation.
- **A derivation the library memoises is memoised for one *sourcing*, never for
  the process, and the kit-root anchoring is the worked instance.** The
  resolution above runs the owning kit's libraries in a **subshell**, which
  inherits the dispatcher's shell variables — a memo guard among them. So a
  library that skips its own derivation on a warm memo hands the bridged member
  the **dispatcher's** value, computed before the consumer config in scope was
  read, while every knob resolved from scratch in the same pass takes the
  config's. The two disagree only where a consumer config narrows the derivation's
  input, which is why it stayed invisible until a fixture pair overrode
  `GATE_SDK_KIT_DIRS` for a member declaring `GATE_KIT_ROOTS_REL`: the shell form
  runs as a fresh process and re-derives, the bridged form does not. The guard is
  therefore cleared where the library resolves it, keeping the memo's saving
  (one anchor derivation per invocation) without its cross-source reach.
- **A knob the bridge can carry is one whose default is `declare -p`-visible
  *after* the owning kit's library has been sourced, so the library is where a
  kit knob's default belongs.** This is a property of the bridge rather than of
  any batch that trips it: the resolver confirms a declared knob by asking
  `declare -p` for it, so a value defaulted inline at a use site
  (`"${KNOB:-<default>}"` inside a check) or inside a helper function's body is
  invisible to it, and the member's very first post-port run takes the
  undeclared-knob refusal — whose message names the bridge rather than the
  missing default, which is what makes the failure expensive to read. The
  discharge is one edit in the library and its cost is per *knob*, so a batch
  whose members share bridge-blind knobs pays it **once, in front of the batch**,
  rather than discovering it once per member inside three ports. The seam holds
  through that resolution rather than being loosened by it: naming a default in a
  kit library is where a kit default already belongs, and a path or a directory
  is not consumer vocabulary — the vocabulary the value points *at* stays in the
  consumer's own files (CLAUDE.md §The provenance seam).
- **A bridged value may not carry an absolute path.** The resolved argv is baked
  **verbatim** into the generated pre-commit hook, which is tracked, so an
  absolute value commits one machine's checkout path to a public file. That is a
  constraint on what a knob may hold, not on the bridge: `gate_kit_roots`
  therefore crosses as `GATE_KIT_ROOTS_HERE`, each root spelled relative to the
  invoking directory, which the binary re-absolutises against its own cwd to
  recover exactly the path the shell compares. `GATE_KIT_ROOTS_REL` is the other
  spelling — anchored at the kits' parent, which is what a repo-relative
  pathspec or a knob-prefix owner needs — and the two are separate knobs because
  the anchor relating them is not recoverable from either alone once an override
  is in play.
- **`GATE_SDK_RESOLVING_KNOB` is the *set* of names under resolution**, one per
  line, exported into the subshell before the kit's libraries are sourced, and a
  reader tests **membership** rather than equality. A knob whose value is a
  consumer *command's output* costs a subprocess to compute, and a library
  resolving every such knob on every source would pay that cost per source. A
  library with no expensive knob ignores it and is unaffected. The set spelling
  is what the per-kit batch below requires: one subshell now carries a whole
  kit's slice, so an equality test would match no batch of more than one name and
  the gated block would compute for nothing. A block whose names are absent from
  the set still does not compute, which is the property the gating buys, and the
  saving is that it computes once per *run* rather than once per requesting
  member. The install
  transport and payload disclosure vocabularies cross this way as index-aligned
  id/pattern pairs (canon-kit/SPEC.md §lib/spec.sh), so a compiled member receives
  the consumer command's *output* and spawns no interpreter of its own. Their
  ownership is untouched: what changes is the transport, not who writes the
  vocabulary.
- That kit's `lib/*.sh` is sourced **in a subshell**, in glob order, so a kit
  library's globals cannot leak into the dispatcher or across members. The value
  is read after every library has been sourced, so a kit whose knob is resolved
  by one library and consumed by another is carried correctly.
- **Resolution is batched by owning kit: one subshell per kit per call, not one
  per declared knob.** The declared set is partitioned by
  `_gate_knob_owning_kit`, each kit's slice is resolved inside a single subshell
  that sources that kit's `lib/*.sh` once, and the elements are re-emitted in the
  **requested** order, so the argv a caller receives — and therefore the tracked
  hook it is baked into — is unchanged by the batching.

  **The batch rests on a property, not on an assumption, and the property is
  stated here because a reader would otherwise have to re-derive it.** A knob's
  resolved value is a function of the knob name and the tree's config **alone**:
  no resolver reads the requesting member, whose name reaches the resolution only
  as text in the does-not-define refusal's message. That is what makes a per-run
  resolution value-identical to a per-member one, and it is what a later arm
  wanting a member-sensitive knob would have to break first.

  **A refusal anywhere in a slice fails the whole call**, exactly as a per-knob
  refusal did: the bridge is fail-closed, and a partially resolved environment is
  the fail-open dressed as a default that the does-not-define refusal exists
  against.

  What the batch buys is the bridge's whole cost, which was one subshell per
  *(member × declared knob)*, each sourcing the owning kit's entire library.
  `--emit`-ing the generated hook resolves the argv for every `tier=precommit`
  member, so it is the loudest reader of that cost and the one the change is
  measured on (§gen-pre-commit).
- The resolved array is serialized **tab-joined**. Whitespace is preserved
  inside an element, which is exactly why the whitespace-separated scalar shape
  cannot serve — `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` contains `Out of scope`. A
  scalar knob is a one-element array; the two cases share one grammar.
- **An associative knob takes a third arm: its key-value pairs, sorted by key,
  one pair per tab-separated element, each pair spelled `<key>=<value>`.** The
  **split is on the first `=`**, so a value may carry `=` freely and only the key
  is constrained — the same rule `env` itself applies one level out, where the
  outer `env` splits the argv element's first `=` to recover the variable name.
  The grammar therefore adds no second parsing convention to the protocol; it
  repeats the one already in it.

  **Which arm a knob takes is derived from `declare -p`, never declared.** The
  bridge already sources the owning kit's `lib/*.sh` and confirms the knob is
  defined; the same `declare -p` output carries the `declare -A` marker, so the
  producer sees the shape without being told it. The asymmetry with the prefix
  arm below is deliberate, and it is the rule a later arm is measured against: the
  `*` spelling in a member's declared knob roster exists because there **is** no
  variable named `EVIDENCE_KIT_RUN_` to inspect, so that spelling carries
  resolution information nothing else holds. A keyed knob carries none — the
  variable exists and answers the question itself. A shape marker there would
  maintain a derivable fact, which derivation-first forbids, and would put the
  declaration and its subject in two places that can disagree. So `--knobs` output
  stays a flat list of names and no `.gate` descriptor field is added.

  **Sorting is load-bearing rather than cosmetic.** Bash associative arrays
  iterate in hash order, so an unsorted emission would make the resolved argv —
  baked verbatim into the tracked generated pre-commit hook — depend on bash's
  hash seed and churn that file for no change. The sort is `LC_ALL=C` for the
  same reason one level further out: byte order is what the compiled reader's own
  sort produces, and a locale-dependent collation would make the emitted hook
  depend on the invoking environment.

  **Two alternatives were rejected, recorded so a later arm does not retry them.**
  *Reusing the prefix-family wire* — decomposing a map into
  `GATE_SDK_KNOB_<NAME>_<key>=<value>` variables — would reuse the prefix
  machinery almost entirely, and it fails on the readers: both keyed members
  iterate the map's **key set**, while the rule below holds that a prefix is a
  resolution set and never a roster, precisely so a stray variable sharing the
  stem is not published as a member. A map has no separate roster knob to fall
  back on, so reusing the family wire would either violate that rule or reintroduce
  the collision it was written against, and it would restrict keys to
  identifier-safe characters for no gain. A single variable carrying its own keys
  has neither problem: the `declare -A` *is* the roster. *An index-aligned sibling
  knob* (`…=<values>` plus `…__KEYS=<keys>`) has live precedent in the
  install-transport vocabularies below, but there the two halves are one consumer
  command's single output, where this would mint a second bridged name per map
  whose alignment nothing enforces and whose halves can be resolved, baked and
  read independently.

  The knob-shaped instances part company on whether a gate reads them.
  `QUEUE_KIT_LESSON_SINKS` has no gate reader — only
  `queue-kit/bin/lesson-sink.sh` reads it — so the queue-kit cohort ported before
  this arm existed and is unaffected by it; it becomes portable now, and is named
  here so a later selector does not re-derive its status.
  `EVIDENCE_KIT_SCENARIO_GLOBS` and `LIFECYCLE_KIT_PREDECESSOR` are each read
  **by key** by a registered gate, `check-evidence-baseline` and
  `check-stage-entry` respectively, and are the arm's live instances: the second
  resolves at every pre-commit run in this repo, the first is empty here and
  exercised non-empty only by its owning fixture.

  **The residue the arm leaves, stated rather than discovered later.** A reader
  taking a knob as an *array* when its consumer has since redeclared it
  `declare -A` receives `key=value` strings and cannot tell. The reverse
  direction *is* caught — the map reader refuses an element with no `=`. Closing
  the remaining direction means transporting the reader's expected shape back to
  the producer, which is the maintained declaration the derived-shape rule above
  declined to mint, and the hazard needs a consumer to change a shipped knob's
  **grammar**, which is a kit-SPEC-governed contract change rather than a
  configuration edit.
- **A declared name ending in `*` is a prefix, and the bridge resolves the whole
  family under it** — one `GATE_SDK_KNOB_<NAME>=<tab-joined>` element per match,
  sorted, so the emitted environment is deterministic and the generated hook it
  is baked into is stable. This is what lets a member read a keyed family whose
  key set is *another knob's value*: `--knobs` publishes a static roster, so
  without it a member can only name knobs it knows at compile time.
  `EVIDENCE_KIT_RUN_<suite>` is the live instance — one variable per suite, with
  the suite set coming from `EVIDENCE_KIT_SUITES`.

  **This is a family of separate variables, not the keyed knob the bullet above
  carries, and both shapes now cross — so the live question is which wire each
  takes and why, never whether one crosses at all.** A prefix family is one
  variable per member, and the *name* carries the key, which is what lets the key
  set come from another knob's value; a keyed knob is one variable carrying its
  own keys, which is what lets its key set be the roster. A member whose roster
  is external takes the prefix; a member that iterates the map's own key set
  takes the keyed arm.

  **Resolution happens at the instant the scalar arm's does** — after the owning
  kit's `lib/*.sh` has been sourced, in the same subshell, which is the load-bearing
  detail: it is what puts a consumer config's *loop-declared* variables in scope.
  `scripts/evidence-config.sh` builds most of its family with a `while` loop over
  `gate_fixture_suites`, so a reader that parsed the file rather than resolving it
  would see the statically-assigned names and silently miss the rest.

  **A prefix matching nothing resolves to an empty family and passes**, and the
  fail-closed obligation it looks like that drops is **relocated, not removed**:
  it belongs to the **reader**, the only party holding the roster. A reader that
  looks up a name its roster named and does not find it refuses, naming the
  member. The bridge cannot make that call, because a prefix carries no
  expectation of its own — which is the same thing the resolution-set rule below
  says. The element-shape refusals apply per match, naming the offending family
  member rather than the prefix.

  **Refusing on an empty match would collapse adopted-but-broken into
  not-adopted's arm.** A roster naming a member with no entry is *adopted but
  broken* and refuses; an empty roster is *not adopted*, performs no lookups, and
  drops its section. `check-enforcement-fresh` is the live case: a consumer that
  has not adopted evidence-kit has no `EVIDENCE_KIT_RUN_` family at all, and the
  shell emitter it replaced dropped the Validate-suites section rather than
  failing.

  **A prefix is a resolution set, never a roster.** It says *resolve values for
  these names*; it does not say *these are the members*. The roster comes from
  the roster knob, and a reader must look each name up rather than enumerate what
  matched — `EVIDENCE_KIT_RUN_ID` matches `EVIDENCE_KIT_RUN_` and is evidence-kit's
  run identifier, not a suite. A reader treating the matched set as the roster
  would publish it as one.
- **Four refusals, each exit 2 naming the knob** (§Fail-closed contract): an
  element containing a **newline**, which would break the line-per-element argv
  protocol; an element containing a **tab**, which would break the
  serialization; a **key containing `=`**, which would make its pair
  unsplittable; and a knob the owning kit's library **does not define**, since
  serializing that as empty would hand the reader an empty set — a fail-open
  dressed as a default, and for a prune set a silently *larger* walk. An
  element-less knob is not that case: a resolved-empty array serializes to the
  empty string and is carried, so *absent* and *empty* part company on the
  reading side too, and an empty map is the same resolved-empty reading.
  On a keyed knob the newline and tab refusals apply to the **key and the value
  of every pair**, naming the offending key rather than the knob alone — the
  shape the prefix arm already uses when it names the offending family member.

  **The newline refusal has a design consequence worth stating as a rule, because
  it decides the shape of every configurable *document*: values cross the bridge;
  documents cross as a path.** A stylesheet or an HTML fragment is newline-bearing
  by construction, so its content cannot ride the bridge at all — what crosses is
  the **relative** path to a directory or file, and the binary reads the bytes
  itself. `check-graph`'s theme seam is the worked instance and the counterpart is
  its vocabulary, whose layer rules are short single-line elements and so cross as
  values (§check-graph).

  **The keyed arm closes a live fail-open rather than merely adding a case.**
  Before it, the resolution checked only that a knob was *declared* and then
  expanded it through a nameref, so an associative knob did not refuse: it
  silently serialized its **values**, in hash order, losing the keys. The
  documented limit was therefore prose-only, and nothing stopped a consumer from
  porting such a gate and receiving a quietly wrong environment. The keyed arm
  removes the hazard **by construction** — the shape is now taken rather than
  fallen through — which is why no separate guard is added for it.

  **The tab refusal is discharged upstream for a consumer-supplied ERE**, which is
  worth recording rather than re-checking per port. A POSIX ERE legitimately may
  contain a literal tab, so a claim vocabulary looks like a knob that could trip
  the refusal; `spec_claim_vocabulary` already rejects a vocabulary line carrying
  an extra tab and its line-oriented read forecloses a newline, so no value
  reaching those knobs can violate the bridge. The constraint is that loader's,
  not a new bound this bridge imposes on consumers.

**The graph family's resolved globals.** `GATE_SDK_GRAPH_ARTIFACT`,
`GATE_SDK_GRAPH_THEME_DIR` and `GATE_SDK_GRAPH_MAX_EDGES` take guarded top-level
assignments here rather than inline `${KNOB:-…}` defaults at their use sites, on
the cause the roster above states — a default `declare -p` cannot find is the
bridge's undeclared-knob refusal — and the first two ride `GATE_SDK_GATES_DIR`'s
own resolved value so each pair stays one value by construction.
`GATE_SDK_GRAPH_EXTERNAL_REFS` is a whitespace scalar feeding an array, the one
case a resolved global earns a spelling of its own, so it resolves to
`GATE_GRAPH_EXTERNAL_REFS` beside `GATE_PRUNE_DIRS`. Every one of these values
stays **relative**: the resolved argv is baked verbatim into the tracked
pre-commit hook, and an absolute value would commit one machine's checkout path
to a public file.

**`GATE_SDK_ROOT_HERE` is the kit's own root as a bridgeable value**, spelled
relative to the current directory by `GATE_KIT_ROOTS_HERE`'s rule and for its
reason. It exists because a compiled member that must reach a file inside its own
kit has **no `BASH_SOURCE`** to find it by — the one thing a shell gate knew about
itself for free is the one thing a port must be handed. `check-graph`'s assertion
D is the worked instance: it spawns `bin/gen-pre-commit.sh`, which stays shell
(§gen-pre-commit). The kit-root set is not a substitute, because
`GATE_SDK_KIT_DIRS` may narrow it to a consumer's own tree — which is exactly the
configuration a sandboxed fixture runs under.

**A consumer config *file* whose content is rule data is sourced here, not in the
member.** `GATE_SDK_GRAPH_VOCAB` (default `<gates-dir>/graph-vocab.sh`) is read at
top level, the shape `GATE_SDK_MSG_PATTERN_FILES` already uses, and its six
globals are defined before the source so the bridge's does-not-define refusal
cannot fire on any of them and an absent file resolves to empty arrays. The
member is compiled and receives the resolved values, so the path itself is a knob
the crate never declares.

The `GATE_SDK_KNOB_` prefix is deliberately **not** the knob's own name: reusing
`CANON_KIT_MANIFEST_FILES` as an env scalar would collide with the existing
whitespace-scalar override convention (`GATE_SDK_PRUNE_DIRS` is exactly such a
knob), and one name meaning two grammars is the defect the prefix avoids.

`env` is an external program in the dispatch path, sanctioned here because the
dispatcher is already bash — this adds no dependency any caller did not have —
and because the alternative, a shell wrapper per gate, would add a bash hop to
every invocation and contradict the port's own purpose. **Its cost is borne by
the callers, and is stated here rather than left for each to rediscover**: a
bridged argv begins with `env`, not with the dispatch executable, so a caller
that needs *the executable* rather than *the command* takes the first element
that is neither `env` nor a `NAME=VALUE` assignment. §run-gate-tests is the one caller that
needs it, and it is named there.

**The sanction is a property of the caller, so it binds only where the caller is
bash.** The battery's own dispatch path is not one: the `--run` arm sets each
child's environment directly, because the condition this paragraph rests on —
*the dispatcher is already bash* — is false there (§run-gates). The prefix holds
everywhere the producer is a bash one, the generated hook's baked argv above all,
and stays a live contract for `gate_command`'s surviving callers.

**The bridge is process-global, and the crate's own tests are serialized against
it rather than disciplined around it.** Its values are environment variables, so
every `#[test]` in one test binary shares one environment: two cases setting the
same `GATE_SDK_KNOB_<NAME>` to different values race, and the loser asserts
against the other's render. The symptom — a test green alone and red in the suite
— reads as flakiness rather than as this contract, which is what made it worth
mechanizing: the defect was probed, failing 1 of 5 suite runs at cargo's default
parallelism and none of 5 serialized.

The crate carries **one** serialization point, `native/src/knobenv.rs`, and the
guard it hands back **is** the write API: `knobenv::lock()` returns a `KnobEnv`,
whose `set` and `remove` are the crate's only spellings of an environment write.
A case holds that guard across its writes **and** across the assertions that read
them; holding it across the reads is the load-bearing half, since a lock released
at the write leaves the render it was taken for unprotected. The two bridge
helpers (§run-gate-tests) take the guard as an argument rather than acquiring it,
so a case that already holds it cannot deadlock against them.

**Its machine side is a unit test in that module**, in the roster shape
§check-reads-couples' unit test B uses: no crate source outside `knobenv.rs`
names `set_var` or `remove_var`. Without it the guard is advice, and a later case
calling the std API directly restores the race silently.

**The rejected alternative is recorded because it is the one that looks cheaper.**
Threading a knob into the code under test as a parameter deletes the race by
deleting the coverage: environment resolution *is* the production path — a gate
invoked outside this bridge refuses on the unset knob — so a test passing the
value in as an argument stops exercising the only resolution the binary ever
performs. Serialization is therefore the deliverable rather than a caveat on it,
and its point applies to **every** `GATE_SDK_KNOB_`-writing case in the crate,
not to whichever one happened to red.

The binary's path is the knob `GATE_SDK_NATIVE_BIN` (§Layout and configuration),
never a literal. An **absent or non-executable** binary when a registry member
dispatches to it is a harness error — **exit 2, never a skip and never a pass**.
This is §Fail-closed contract applied to dispatch: the failure a skip would
create is a battery that silently stops running a gate whenever a build is
missing, which is the worst available outcome. The ruled install model leaves
this exactly as it is and makes it unreachable in a correctly installed tree —
a member arrives either with its verified artifact or not at all, omitted from
`gates.list` and recorded there (§Porting a gate to the binary substrate,
criterion 5) — so exit 2 is the backstop for a tree whose binary was deleted or
replaced, never the path an unsupported platform takes. A binary that is
executable but **cannot report its knobs** — a non-zero `--knobs` — is the same
harness error for the same reason: a bridge that quietly carried nothing would
hand every declared reader an unset variable.

`fail_closed` must be passed *only* a status that genuinely means the check
could not execute (an awk/jq/parser crash) — never `grep`'s exit 1, which is
the expected "no match"; the caller draws that line at the capture site.

### lib/inject.sh

The marker-bounded span mechanics every kit's agent-file injector shares — three
functions over one notion of a well-formed block.

**A compiled counterpart exists, and it is a divergence rather than a
translation.** `native/src/marker.rs` carries a read half and a write half for
the crate's own block consumers: the read half is the single implementation the
value-rollup comparator and the value-rollup generator both use, extracted from
the comparator's private copy when the writer landed. The write half matches this
library on a marker hit — markers matched whole-line, content between them
replaced, the file otherwise byte-untouched — and **tightens it on a miss**:
where `inject_marker_block` appends a fresh block when the begin marker is
absent, the compiled writer **refuses**, and likewise on an unbalanced or
reversed pair. The refusal direction is deliberate, because the failure it
prevents is the expensive one: a generator that appends when it cannot find its
markers corrupts a hand-authored page, and the freshness gate then reports the
corruption as staleness rather than as damage.

**This library is not retired by that, and the duplication is the ordinary
transitional state.** Its append-on-absent behaviour is still correct for its own
callers — the lifecycle installer's attribute and registration blocks among them
— which are shell and unported. So the same miss reads two ways depending on
which implementation reaches it, until each remaining caller ports in turn;
retiring `lib/inject.sh` belongs to whichever unit ports its last one. Recorded
so a later reader does not take the two halves for copies of each other.

`inject_marker_block <file> <begin> <end>` takes the inner block content on
stdin. It writes `<begin>` + the piped content + `<end>` into
the target: replacing the span between an existing marker pair (inclusive) in
place, or appending a fresh block when the markers are absent, so a re-run
never duplicates. A begin marker without its end is a malformed target — it
refuses (exit 2) rather than guess the bounds; a missing target file is exit
2. On success it echoes the action taken (`appended`|`replaced`) for the
caller to report.

`read_marker_block <file> <begin> <end>` is the retrieval half: it prints the
existing block's inner content (markers exclusive) on stdout, prints nothing
and exits 0 when the marker pair is absent, and refuses on the same two
malformed targets its sibling refuses on — a begin marker without its end, and
a missing file, both exit 2.

`remove_marker_block <file> <begin> <end>` is the third: it deletes the marker
pair and everything between it (inclusive), leaves the rest of the file
byte-identical, and echoes `removed` when it removed one. An **absent begin
marker is a no-op that exits 0 and prints nothing**, not a refusal — the caller
is a path reversing an installation, which cannot know whether the block was
ever written there, so a second removal must be as quiet as the first. It
refuses on the same two malformed targets its siblings refuse on, both exit 2.

All three agreeing on what a malformed target is, in one module, is the point of
keeping them together: a caller that reads a block before rewriting it, or
removes one it may never have written, must not meet three different answers to
the same question.

The module owns **placement, retrieval and removal, and nothing above them.**
Block content *generation* stays with the caller (the lifecycle roster, the doctrine
digest), and so does any rule about what to *preserve* out of a block that was
read — doctrine-kit's declared-trim round-trip (doctrine-kit/SPEC.md
§install-doctrine) is read-compute-emit in that installer, not a preserve rule
pushed down here. A preservation parameter on `inject_marker_block` would ship
one kit's marker vocabulary to every consumer of a generic injector, which is
the provenance seam this split keeps intact; the caller-side shape is the one
`bin/gen-pre-commit.sh` already uses for its `gen=manual` regions. So a second
injector adds no second copy of the awk replace logic — every marker-bounded
projection in the tree rides them, `doctrine-kit/bin/install-doctrine.sh`
and `lifecycle-kit/bin/install-lifecycle.sh` among them. A sourced library, not
a gate: exercised end-to-end wherever an installer that rides it runs
(doctrine-kit and lifecycle-kit `smoke/install.sh`) — doctrine-kit's covers the
read half through the trim round-trip its acceptor drives, and the removal half
through a `--remove`/reinstall round trip beside it.

### lib/declaration.sh

The tightened-gates declaration grammar — **two container arms over one token
predicate**, sourced by three callers. The token predicate is a bare gate name;
the container is the only thing that differs between the arms,
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
unparsed-and-not-`None`. Every arm has a named reader at a named transition, and
one with none would be removed: *absent* is read by `check-release-bump` at its
fixed-section presence assertions and by `check-tightened-gates-grammar` at its
status-2 arm; *explicit `None`* by the grammar gate's `none` counter, which its
clean line prints, and by note-parity as the resolved empty set it compares;
*tokens* by all three, as a bullet count in the bump gate and as a token set in
the other two; *unparsed* by the grammar gate's finding list and by note-parity's
refusal. The third arm is the one that earns the helper. A
non-`none` section yielding zero tokens must not become "no allowed reds": on a
green battery that passes silently over a note naming several gates, and on a red
one it fails *loudly with a false message*, accusing the note of an omission it
did not make. The assertion is not leniently disarmed there — it is severed from
the artifact it claims to read. So the helper refuses, and the refusal is what
closes the class permanently: no future markup variant can disarm the assertion,
only red it.

**The library is dual, and one live caller is what makes that true rather than
asserted.** `bin/upgrade-smoke.sh` at its declaration-resolve step uses both arms
(§upgrade-smoke) and is the shell form's **only** caller since §The declaration
cohort: the members that shared it ship as compiled subcommands and reach
`native/src/declaration.rs` instead. The shell caller set therefore does not
empty, so criterion 6's delete-the-shell-form outcome is unavailable and the
duplication takes the machine-held disposition (§The port-candidate criteria,
criterion 6).

**The standing oracle that discharges it.** A port-time byte-identity proof is
not machine-held — it proves the two agreed once and expires at the next edit to
either side, which is exactly the failure that clause names. So the binary
carries a top-level `--declaration-parity` arm reporting the compiled holder's
*classification* of one input, one record per line, and
`gate-tests/declaration-lib-parity.test.sh` — in the fixture-runner battery,
resolving the binary through `GATE_SDK_NATIVE_BIN` — feeds one canned corpus to
both holders and compares byte for byte. A flag rather than a subcommand, for the
reason its siblings are one (§check-gate-substrate-parity). The corpus is the
**trichotomy** rather than a sample of any tree's notes, so it reaches every arm
both holders have, including the container that is neither `None` nor
token-bearing and whose status 1 comes with *empty* stdout.
`gate-tests/lib-declaration.test.sh` keeps its place unchanged: it is the shell
arm's own runtime lock-in and fails when only the shell is wrong, which a
comparison of the two structurally cannot.

**One conflation both holders now carry, recorded rather than repaired.** On the
refusal path each arm emits the tokens it had already resolved *before* the
offending lines, because the shell form prints a token as it walks and appends
the offenders at the end — so a container mixing a readable and an unreadable
bullet reports the readable one to its caller's finding list as unreadable. The
verdict is unaffected in both holders and only the diagnostic list is wrong; the
compiled form reproduces it, because a port proves parity and does not fix the
rules it ports. Repairing it is a two-holder edit in one unit or the parity test
reds, which is the oracle above doing its job.

The remaining caller relations, stated in one place: this repo's
`check-tightened-gates-grammar` uses the markdown arm's verdict at each note it
walks; `check-tightened-gates-note-parity` uses both arms, comparing a note's
`Tightened gates` section against its declaration-file argument's record set; and
`check-release-bump` uses the markdown arm's *container* alone, counting bullets
across the note's declaration-bearing sections. That last caller is why the container and the token predicate are
separable rather than one pass: Behavior-changes lead tokens are legitimately
prose phrases, so the bump derivation needs the bullets without the token
predicate. Before this helper the container was stated three times and two of
the statements already disagreed on whether a bullet marker could be indented,
so the section a bump was derived from and the section an allowed-red set was
parsed from were not guaranteed to be the same section. A sourced library, not
a gate, so it owes no `good/`+`bad/` pair. The record arm is exercised a second
way, through `scripts/gate-tests/check-tightened-gates-note-parity`'s own
`good/`+`bad/` pair, whose `tightened-gates.txt` fixture drives it via the gate's
declaration-file argument — which since the port drives the *compiled* holder,
so that pair proves the shell arm only through the parity comparison above and
never instead of it.

**The compiled holder's public surface is bounded and is exactly these three
entry points** — the container arm alone, the markdown arm's verdict, and the
record arm. There is no writer, no renderer and no section-discovery API, and
adding one is a design decision with its own reader rather than an omission to
fill in. Its consumers are four and are named: the three gate modules of §The
declaration cohort plus the parity arm above. `bin/upgrade-smoke.sh` is
deliberately not among them, which is the criterion-6 ruling and the reason the
oracle exists.

The helper carries no section name and no gate name of its own — both are the
caller's arguments, and it takes no configuration. That is where the seam falls:
the parsing is kit mechanism, the parsed content is the consumer's. **The seam
falls in the same place on the compiled side and is checkable there**:
`native/src/declaration.rs` takes its section as an argument too, so the four
published section names live in the three gate modules that pass them — this
consumer's rule content sitting in this consumer's rules — and a grep of that
module for any of them returns nothing.

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

**The runner is two halves, and which half owns what is the first thing to know
here.** The **arm** is `--run`, a bridged non-gate arm of the binary (§The
non-gate arm): it owns the registry walk, both selectors, the dispatch, the
worker pool, the timings, the omission accounting and the output contract.
`bin/run-gates.sh` is the **front-end**: it resolves the repo root, sources
`lib/gate.sh`, parses argv, resolves the arm's bridged environment in **one**
`gate_knob_env` call, and `exec`s the binary — the shape `--emit` already had,
which is the precedent the runner's own arm is built on. Everything the argument
grammar below specifies is the front-end's; everything the output contract
specifies is the arm's.

**The front-end keeps a shell dispatch loop for one branch, and the branch is
criterion 5's.** A host the payload carries no verified artifact for is an
*omit-and-declare* install (installer/README.md §The gate binary): its
`gates.list` records the lost members as comments and keeps its shell ones, and
its battery has to stay green — an asserted contract, exercised by the
installer's binary-less leg. So where `GATE_SDK_NATIVE_BIN` names nothing
executable the front-end dispatches the registry itself, and where it names a
binary — every covered platform — the arm does. This is the one duplication the
port carries, it is admitted on criterion 6's *unless* clause, and it is held by
an **executed comparison** rather than by care: the two dispatchers' transcripts
over one hermetic registry are required byte-identical, banners, tails, omission
line and summary alike (§run-gate-tests). `--emit` has no such fallback and
refuses when the binary is absent, because an emitter has no shell half left to
run.

**The split exists to delete a per-member bash bridge, and the arithmetic is why
it was worth a port.** The front-end resolves the declared-knob union once for
the whole run where the loop it replaced resolved one member's knobs at a time —
measured on this tree, `gate_command` over all 106 members and executing none of
them cost 5019 ms before the batched bridge (§lib/gate.sh) and 2714 ms after,
against a whole warm battery of `TOTAL 24990` ms; the arm pays it once. **The
outcome, measured warm on a 14-core host, median of three:** the battery is
7756 ms at the default worker count and 18325 ms under `GATE_SDK_JOBS=1`, where
the shell dispatcher it replaced was the 24990 ms above. The serial figure is the
bridge saving alone and the gap to it is the pool's; both are bounded below by
the longest single member, which is what the timings file's slowest rows name.

**What the completion predicate covers, and the one bash process it does not —
ruled 2026-08-23 by the operator.** TRAJECTORY.md §The closed rulings states that
*port complete* means "the battery runs from the hook to the binary with no bash
in between", naming this unit. That predicate governs the **hook-to-binary path**
only. The retained knob resolver is one bash process per battery run — the
config bridge's own producer, which §lib/gate.sh rules is the single place a
knob's value is computed — and this unit owns its **cost**, not its existence.
The cost is what fell: one subshell per owning kit rather than one per declared
knob. Recorded here because the predicate and the surviving process would
otherwise read as a contradiction to anyone checking the claim against the tree,
and the resolution is a scope reading rather than an unmet condition.

**A member dispatches as a child process, and the threads are the concurrency.**
A compiled member is a function in the same binary, so calling it in-process is
available and looks free. It is refused on three grounds, and the measurement
prices what the refusal forgoes at well under one percent of the saving:

- **It would silently retire the declared-knob discipline.** A gate reads its
  knobs from the process environment. One shared environment carrying the union
  means a member reading a knob it never declared would *succeed* — and
  `--knobs` exists precisely to hold "the crate declares only the knobs its own
  code reads" to executed behavior. That is the bridge's does-not-define refusal
  failing in the other direction, with nothing to catch it. So the arm strips
  every inherited `GATE_SDK_KNOB_*` from a child and re-adds only the ones that
  member's registry entry declares, prefix families included.
- **It would lose fault isolation.** A member that panics or aborts must red
  *that member*; in one process it takes the battery with it.
- **It would fork the runner in two.** Members resolving to `.sh` survive, and
  consumer shadowing of a ported member with a shell script is a permanent
  contract (§lib/gate.sh), so a dispatch path that only reaches compiled members
  needs the exec path anyway and buys a second implementation rather than
  replacing one.

**`env` leaves the battery's dispatch path.** The arm sets each child's
environment directly rather than prefixing an `env` argv element; §lib/gate.sh's
sanction for `env` is that "the dispatcher is already bash", and this dispatcher
is not. The prefix stays in the generated hook's baked argv, which is still
emitted by a bash producer (§gen-pre-commit).

**A child's two streams merge into one capture file rather than two pipes**, and
the merge is two handles on one file description, so they share an offset and
interleave exactly as the shell dispatcher's `2>&1` did. Reading them as two
pipes would reorder a member's own report against its own diagnostics. **Its
stdin is `/dev/null`**: under a worker pool an inherited terminal is a shared
resource two concurrent members could both read from, which is the same class of
interference the per-gate scratch isolation below exists against.

**A child killed by a signal reports `exit 128+n`.** That is the spelling bash's
`$?` gave the shell dispatcher this replaced, so the tail grammar
`scripts/parse-gates-log.sh` reads keeps one shape and the port mints no fourth
tail. The tails are `(exit N)`, `(dispatch harness error, exit 2)` and
`(unresolved)` — cited rather than counted, because a later dispatch shape may
add one.

The output contract is **quiet green, loud red**. A passing gate prints
nothing: its captured output is discarded and the run ends with the summary
line alone, whose executed-gate count (`All N gates passed.`) is the
roster-collapse tripwire — a battery that silently shrank shows a smaller N. A
failing or erroring member prints its `===== <name> =====` banner and its
captured output verbatim, always — the red path is the feedback channel and
never quiets.

**A declared omission is what keeps that tripwire honest.** A member the
installer omitted because no verified binary reached this platform is recorded
in the registry as `# omitted: <name> <reason>` (installer/README.md §The gate
binary) — a comment line, so `gates_list_members` strips it and N shrinks
legitimately. The runner counts those lines and prints the count and its remedy
beside the summary, one line per reason token present, so a declared omission
stays distinguishable from the regression the tripwire exists to catch. **The
line is separate from the summary and carries none of its text**, and that is
load-bearing rather than tidy: `run-consumer-smoke.sh` and the installer's own
smoke both match the green phrase against this output, so a remedy folded into
the summary line would either break their assertion or make the phrase match on
a run that omitted half the battery. A registry with no omission lines prints
nothing extra, so the zero case adds no output at all. `GATE_SDK_VERBOSE` (any non-empty value) restores the full banner
roll, the on-demand reading for the vacuous-pass tripwire (a "0 files scanned"
clean line is visible only in the gate's own banner). Env over flag by the kit
convention: one mechanism serves the interactive run, the generated hooks, and
any CI wrapper without an argv contract change. Gates themselves are untouched —
each still prints its single clean line per the output contract (§Output
contract); the runner captures it.

**That convention governs configuration, not selection, and the bound is ruled
here** rather than left for a later reader to re-litigate against the sentence
above. `GATE_SDK_VERBOSE` is *how* the battery reports — a value every caller of
the battery should carry uniformly — and env is right for it. *Which* members run
is the one thing that must not be ambient: a selector spelled as a `GATE_SDK_*`
knob would be inherited by the generated hook, by `run-consumer-smoke.sh` and by
any CI wrapper that ran under it, silently narrowing a battery to one member
while the summary line still reported a pass — precisely the
green-with-nothing-behind-it the declared-omission accounting above exists to
make impossible. A selector spelled in argv dies with the process that typed it.
So the runner draws the line where the two selectors below sit: reporting is env,
selection is argv.

**`GATE_SDK_JOBS` sits on the reporting side of that line, and the placement is
argued rather than assumed.** It is the worker count: unset resolves to
`std::thread::available_parallelism()`, and `1` restores a serial run, which is
what makes a suspected interference reproducible without a rebuild. It is env
against the sentence above because worker count changes no member's
*membership* — it is execution configuration, the same class as
`GATE_SDK_VERBOSE`, and an ambient one narrows nothing. It is deliberately **not
bridged**: it is gate-sdk's own execution config, read once from the environment
the front-end already exports, before the first member is dispatched.

**The concurrency contract.** Members run on a worker pool built from
`std::thread` and `std::sync` alone — **no new crate dependency**, because
objective 4 makes footprint a cost paid per target on every adopter's machine
and a scheduler crate is a cost the standard library already covers. Four
properties bind:

- **Deterministic output ordering.** Each member's captured output is buffered
  and flushed in **registry order**, never completion order, so a red reads the
  same way twice and a transcript diffs against itself. The summary's failed-set
  is likewise registry-ordered.
- **The timings file is not a contended writer.** Per-member elapsed times are
  collected in memory and written once after the join, in registry order,
  `TOTAL` last — the grammar drift-kit's `kpi-gate-runtime` member and its
  collator read, unchanged. `TOTAL` is the **sum** of
  per-member times, as it always was, and therefore stops approximating
  wall-clock: under a pool the two part company, and per-member times themselves
  rise with CPU contention while the run gets shorter. Stated because those two
  readers would otherwise read a rising `TOTAL` as a regression.
- **Per-gate scratch isolation, split by what the directory is for.** Each child
  gets a private `TMPDIR` under the run's own scratch, which is where a member's
  *anonymous* temporaries land. That scratch sits under the **system** temp dir,
  where an anonymous temporary already went, and it is **absolute**: a child's
  working directory is its own, and a relative `TMPDIR` silently resolves
  somewhere else inside one — the defect the crate's own `awk` cross-check caught
  when it ran under a child whose cwd was the crate root.
  `GATE_SDK_TMP_DIR` stays **shared and is not
  isolated**, because it is the home of a declared, content-keyed cache whose
  whole value is surviving the run — `check-crate-arms` writes its
  `crate-arms-<hash>.green` there and reads it on the next battery, and a
  per-member scratch would silently retire it. The rule that states, and which a
  later member is measured against: **an anonymous temporary is private; a
  declared cache is shared and its filename must carry its key.** The run's
  scratch is removed on the way out, so a battery leaves the tmp dir's file set
  exactly as it found it.
- **The projection-ordering constraint is discharged by an invariant, not by the
  scheduler.** A full run over this tree leaves `git status --porcelain`
  byte-identical and the tmp dir with no new entry: no registered member
  regenerates anything another reads, because the freshness family compares
  against an in-memory render rather than writing one. That is the invariant a
  member must not break and the scheduler assumes — a member that wrote a
  projection another member read would be a **defect against this contract**,
  not a scheduling input.

**What the pool did NOT need, priced so a later session does not re-reach for
it.** The standing counter-proposal to a pool is **one shared tree walk feeding
many readers** — several members each listing and reading the tracked corpus
independently, hoisted into a single walk the way a single binary makes easy.
Measured on this tree rather than argued: the tracked corpus is 1634 files and
8.2 MB, **one whole-tree listing costs 1 ms and listing plus reading every byte
costs 6 ms**. Five members do a no-pathspec whole-tree walk (`check-md-refs`,
`check-spec-pointer`, `check-reads-couples`, `check-gate-exemption-tasks`,
`check-gate-binary-fresh`); the rest of the `ls-files` call sites are either
scoped to a scan root or are `--error-unmatch` existence tests, which are
membership queries and not walks at all. So the whole shareable quantity is
about **30 ms of CPU across the battery, overlapped**, against a 7408 ms run.
**The two readings that make it moot rather than merely small.** *One*: for the
members that walk, the walk is not the cost. Behind a single bridge resolution
(~640 ms, §lib/gate.sh, and the floor every single-gate invocation pays)
`check-tree-terms` spends ~494 ms and `check-prose-enum` ~351 ms on their own
per-file matching, of which the shared 6 ms read is a little over one percent —
and that matching is exactly what a shared walk cannot share, since each member
wants a different predicate over the same bytes. The other three sit at or below
the bridge floor's noise. *Two*: the battery's wall clock is floored by a single
member. `check-shellcheck` is ~6417 ms of a 7408 ms run, so the headroom for
every non-shellcheck optimization combined is about one second, and a shared
walk is a low single-digit percentage of even that. This confirms the
critical-path prediction made when the pool was still unbuilt — splitting
`check-shellcheck`'s own corpus is what would break the bound, not the
scheduler and not the walk.

**What the pool actually exposed was a different class, and it is recorded here
because the contract above would not have caught it.** A member that abandons an
**in-process pipe producer** — `printf '%s\n' "${set[@]}" | grep -q …`, the
membership idiom — takes the producer's `SIGPIPE` as the pipeline's status under
`set -o pipefail`, and the verdict flips. Serially the producer's single write
almost always completes before the consumer short-circuits; under contention it
does not, and `check-gate-substrate-parity` reported a descriptor the binary
plainly carried, at roughly one run in three with the crate rebuilding beside it.
This is the **same class** the dispatch capture above exists against, reached
through an array rather than through argv, and the fix is the same shape:
membership is a `for` loop, never a pipeline. **The rule a member is measured
against: a short-circuiting consumer may not be fed by a producer inside the same
gate.** The find is worth more than the fix — a pool does not create such a
defect, it converts a silent one into a visible one, which is what a battery
should do.

**The dispatch capture holds the two streams apart, and it is `gate_command`'s
contract rather than the battery's.** `gate_command`'s stdout *is* the invocation
argv, one element per line; its stderr is diagnostic text. Merging them makes any
stderr a successful call emits the **first argv element**, which the caller then
execs — a 127 naming the diagnostic instead of running the gate, and a status of
0 throughout, so nothing upstream reads as an error. So a caller captures stdout
into the argv value and stderr apart from it.

**The arm is not one of those callers, and keeps no capture file under the tmp
dir.** It builds argv **as data** and never parses a stream into it, so the
failure class is structurally absent there, and the runner's shared
dispatch-stderr file is retired with the branch that read it. The front-end's
binary-less loop still *is* a caller and still splits the two streams, into a
scratch file of its own that nothing outside that loop reads. The rule stays here
for every surviving caller — that loop, the hook generator, the fixture runner,
the hook installer, the consumer smoke, the hermetic-test library, the
stage-entry step and the exec shim among them — each of which parses
`gate_command`'s stdout as argv and must hold the two streams apart.

The failure this closes was neither theoretical nor a race: a helper that
abandoned a pipe producer mid-write emitted a broken-pipe diagnostic on **every**
run wherever SIGPIPE is ignored — a CI runner inherits `SIG_IGN` from its
supervisor, so the write returns `EPIPE` and bash reports it — while a shell at
the default disposition loses the producer silently and shows nothing at all. An
environment-determined defect a green local battery cannot see is exactly what
this capture must be structural against, rather than left to each producer's
discipline: a caller execs whatever stdout carries, so *no* producer may be
trusted to keep stderr clean. Reproducing it locally means restoring the
signal environment (`trap '' PIPE`), which is what makes the
`lib-gate.test.sh` arm deterministic instead of environmental.

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

`run-gates.sh --only <name> [<name>...]` is the **name-keyed** selector — the
ergonomic form of the targeted run that §Layout and configuration's `gates.list`
bullet composes out of two knobs, and thin precisely because that composition
already proves selection needs no new mechanism. It resolves the registry as a
bare run, then runs the named members. `--only` is recognized as the first
argument on the same terms as `--emit` and `--for`, consuming every remaining
argument as a gate name, so the `[gates-dir]` positional is unavailable in this
form: a caller selecting within another registry composes it through
`GATE_SDK_GATES_DIR`, the knob that positional shadows anyway. An empty name
list is a refusal — `run-gates: --only needs at least one gate name`, exit 2,
matching `--for`'s message shape.

Selection is **set-shaped and registry-ordered**: the named set is intersected
with the registry and run in registry order rather than argv order, so two names
give the same transcript whichever way they were typed and the run reads as a
narrower bare run; duplicates collapse silently, because the argument is a set.
**An unregistered name is a refusal, exit 2, naming both the name and the
registry path.** That is the one place `--only` deliberately diverges from
`--for`, and the divergence is the point: an ungoverned path is a fact about the
tree, so the path-keyed selector notes it and exits 0, whereas a *name* is a
claim about the registry and a wrong one is a typo or a stale memory — exiting 0
there would print `All 0 gates passed.`, the vacuous green the summary line
exists to make impossible. A `mode=staged` member receives **no** positional
arguments under `--only`, which is the bare-run behavior rather than the hook's:
`--for` hands such a member its matching paths because it *has* paths, and
`--only` names gates and has none, so the member runs over its full corpus.
Everything downstream is untouched, which is the whole value of siting this on
the runner rather than in a second tool — the `GATE_SDK_KNOB_*` config bridge,
the consumer-first resolve-dir order, the worker pool, the per-gate timing,
`GATE_SDK_VERBOSE`, the declared-omission line and the output contract all behave
exactly as in a bare run. The summary line's `N` is the **selected** count, as it
already is under `--for`; the roster-collapse tripwire that count serves is a
property of a bare run, and neither selector claims it.

**Where each refusal lives is a consequence of the split, and the shapes are
preserved whichever half prints them.** The **front-end** owns every refusal
decidable from argv alone: the two empty-list refusals, the unrecognized-option
refusal, and a leading-dash name inside `--only`. The **arm** owns every refusal
that has to read the registry — `no registry at <list>`, the `--only` steer
beside it, `<list> names no gates`, an unregistered `--only` name, and a member
`--for` cannot resolve. Each keeps the `run-gates:` prefix it always had, because
the message shape is this tool's documented surface and the arm is what the
front-end exec'd; a caller cannot tell which process printed it, and none should
have to.

**The runner honours §The bin/-tool contract**, which binds because its
positional is free text — a path — and an arity check is not a shape check. `-h`
or `--help` as the first argument prints the usage on **stdout** at exit 0; a
first argument beginning with `-` that is none of `--emit`, `--for`, `--only`,
`-h`, `--help` or `--` is a refusal, usage on **stderr** at exit 2, naming the
unrecognized option; and `--` ends option processing, so a gates-dir legitimately
spelled with a leading dash stays reachable. A name beginning with `-` is an
unrecognized option wherever it stands, so `--only --for` refuses at the name
rather than taking it for a gate and calling it unregistered. This is convergence
on a contract the family already carries, not a new design: before it,
`run-gates.sh --help` fell through to the positional and died with `no registry
at --help/gates.list` — a message about a missing file in answer to a rejected
argument, which is why a wrong guess cost three steps instead of one.

**A gates-dir that is really a gate name steers to `--only`.** When the
positional holds no `gates.list` and names a member of the **default** registry
— resolvable through the bridged `GATE_SDK_GATES_DIR` precisely because the failing argument
was never a registry path, which is why the positional crosses to the arm as an
explicit argument rather than as an override of that knob — the refusal gains a line naming the gate and the
flag that runs it. It is an addition to a refusal and never a fallback: the
positional keeps one meaning, and a caller who typed a name gets a remedy rather
than a run it did not ask for, which is the shape a gate's own `help:` line
takes applied to a `bin/` tool's argument error. Where the default registry is
itself unresolvable the plain message stands unchanged, so the steer adds no
failure path of its own.

### run-gate-tests

Golden-fixture runner. Each `<tests-dir>/<gate>/` holds `good/` + `bad/` case
dirs; the runner `cd`s into the case dir and invokes the gate with the args in
the case's `args` file: `#` lines are stripped and the surviving text is
word-split on whitespace into argv, **not** taken one argument per line — so an
argument containing a space is unexpressible in this file, and any second
implementation of the runner (the crate's own parity test among them) must
reproduce the splitting rule, not guess it. `good/` must exit 0 (and, when
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
deleted, never after.

**The two argument positions fail in opposite directions, and the second one's
symptom does not name its cause.** `$1` is the tests dir and is fail-closed — an
absent tree exits 2 saying so. `${@:2}` is the gate-declaration dir set and
**replaces** the resolved default rather than extending it, with a `[[ -d ]]`
filter that drops a non-existent member silently. Pass a checks dir that is not
there and the set is empty, so every gate in the tree reports
`HARNESS: <gate> resolves in none of:` with an empty list after the colon — one
pair of lines per gate, no line naming the argument. The reading is that the
runner's search path was emptied, never that a fixture is malformed.
It bites a consumer whose gates are all `.gate`-declared beside their scripts'
former home: this repo's own consumer remainder keeps no `checks/` dir at all, so
its roster line (README.md §This repo, governed) passes the tests dir alone, and
supplying a plausible-looking `scripts/checks` produces the empty-list symptom
across the whole tree. Filed for a fail-closed second position as
`fixture-runner-checks-dir-fails-open`.

**Every subdirectory of a tests dir is read as a case pair.** The runner globs
`<tests-dir>/*/` and demands `good/` and `bad/` under each, so a directory that
is not a gate's fixture pair is not ignored — it reds as a missing case dir. A
bespoke `*.test.sh` therefore keeps its corpus inline (or under the scratch dir)
rather than in a sibling directory beside itself.

**A bespoke `*.test.sh` reaches its gate the same way, through `gate_run`**
(`lib/test-hermetic.sh`): it names a gate and a checks dir, never a script path,
so a member's port leaves its behavioral tests untouched. Two mechanics make
that work where a `"$GATE"` path did not. The binary is pinned absolute, because
these tests run their gate from a sandbox cwd where the knob's repo-relative
default resolves to nothing. And a case's environment is applied with `gate_env`
inside the command substitution rather than with `env`, because `env` cannot
invoke a shell function and — more to the point — a bridged knob is resolved
when the **argv** is built, so an override set around the binary would arrive
after the value it was meant to change had already been read.

**A `*.test.sh` may also be a *scenario* runner rather than a gate driver, and
one that compares substrates asserts exactly where a dispatch exists.** A bespoke
test whose subject is a shared derivation held on both substrates — queue-kit's
parity harness is the first (queue-kit/SPEC.md §lib/queue.sh) — reaches the
compiled side through `gate_native_bin` rather than a gate name, since the arm it
interrogates is a binary-level flag and dispatches no gate
(§check-gate-substrate-parity). Its skip predicate is
`check-gate-binary-fresh`'s **a declaration is not a dispatch**, applied one layer
up: where the binary is absent or non-executable — a consumer on an uncovered
platform, vendoring the shell library with no artifact behind it — the comparison
has nothing to compare and **skips, saying so on its clean line**, in the
reporting shape the port's omitted-member roster uses, so a reader can tell *no
binary here* from *parity holds*. A silent skip would be the same vacuity the
fixture pair exists to end, arriving in the test layer. Two boundaries keep the
skip narrow. A binary that is **present** and refuses the arm is a stale binary,
not an absent one, so it fails rather than skips and §check-gate-binary-fresh is
what names it — with the same bounded version-skew residual assertion B states,
an adopter vendoring a kit newer than its installed binary reding until the binary
is upgraded. And the runner asserts that its **corpus still reaches the branches
the comparison was bought for**: an agreement over a corpus that classifies
nothing is a pass with no content, which is the skip's failure mode wearing a
clean line.

The runner is the **one caller that needs the dispatch executable rather than
the whole command**, so it is the one the config bridge's argv shape reaches
(§lib/gate.sh). It takes the first element that is neither `env` nor a
`NAME=VALUE` assignment, and applies both the executable guard and the
absolutization to *that* element. Both halves matter: `env` is a PATH lookup
with no directory to resolve, so guarding argv[0] would reject every bridged
member; and the binary knob's default is deliberately a repo-relative path, so
the element that is not absolutized before the `cd` is resolved against the case
dir and vanishes.

**The same split governs where a bridged member's knob *values* come from, and
it lands on the other side.** The dispatch executable and the gate dirs are
resolved at the invoker's root; the **knob values are resolved inside the case
dir**, because a kit library resolves its `<KIT>_CONFIG_FILE` cwd-relative and
the case's own config is what the shell substrate reads. Resolving them at the
invoker's root instead hands the binary *this repo's consumer config* while the
script beside it reads the fixture's — which is not one oracle over two
substrates but two oracles, and it silently contradicts the hermeticity clause
below. So `gate_command` is invoked from within the case dir, and only the
config lookup moves: the binary is absolutized before it, for the reason the
paragraph above gives. A member declaring no knob is unaffected by construction,
the bridge emitting no `env` prefix for it at all.

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

That `cd` buys the **read** side and costs the **write** side, so the runner pays
the write side back: `GATE_SDK_TMP_DIR` is absolutized at the invoker's root and
handed to the **case invocation**, for the mirror of the reason the dispatch
executable is. A member's runtime scratch — `check-crate-arms`'s source-stamp
cache is the live instance — resolves that knob's deliberately repo-relative
default against whatever cwd it has, and under the `cd` that cwd is a **tracked**
fixture corpus: the gate deposits state inside the very directory it is the
oracle for, where it is `.gitignore`d, survives the run, and rides `cp -R` into
anything that vendors the kit tree verbatim. The fix belongs here and not in the
member, because the member's cwd-relative spelling is what §Layout and
configuration ratifies for every inline reader; what is anomalous is a cwd
pointed at tracked content, and the case invocation is the only place this
runner points one there. Pinning the knob per fixture instead would be one
hand-remembered copy per scratch-writing gate, audited by nothing.

The pin's **scope is the pair loop, never the runner's own process**, and that
boundary is load-bearing rather than tidy. A bespoke `*.test.sh` runs at the
invoker's cwd (above) and builds its own sandbox trees off exactly this knob's
relative default; a process-wide export replaces every one of those sandboxes
with the invoker's live scratch, which is how
`evidence-kit/gate-tests/producer-lock.test.sh` — whose scratch trees each own a
`.tmp/run-validate.lock` — comes to contend with a real `run-validate` producer
running in the same tree. Trading a write into the corpus for a write into live
state is not a fix. Within the pair loop the pin stays one-directional: an
already-absolute value passes through, and a case config naming its own scratch
still wins, config being sourced ahead of the environment (§lib/gate.sh).
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
consumer's `git status` shows changes only under the kit roots) and then, over the
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

**The determinism assertion is measured between phase A's two steps, and that
ordering carries the assertion.** Phase A syncs the kit directories, then
regenerates the generated artifacts; the claim under test belongs to the *sync* —
that replacing kit directories wholesale loses nothing a consumer owns. Read
after the regen instead, one `git status` mixes two authors, and the mixture then
has to be unmixed by an allow-set naming every generated artifact. That set was a
hand-held roster with no way to learn an emitter had grown an output:
`gen-pre-commit --write` began writing a second hook, and the roster reddened on
an artifact the contract's own step had just written — an emitter's ordinary
evolution reported as an upgrade regression. It also could not tell a sync that
*clobbered* the agent file from `install-doctrine` writing it, and exempted both.
Reading before the regen removes the roster rather than deriving it, and narrows
the claim to exactly its subject. What that gives up is any claim that the regen
steps write only an expected set — never this suite's claim to make. Each
emitter's write set is held by that emitter's own freshness gate (`check-graph`
assertions D and E cover both generated hooks and the graph artifact) and its own
fixtures; restating it here bought nothing and rotted on the first change.

**The regen step reads the artifact's path rather than spelling it, and reads it
in the *consumer's* library.** The graph emitter is a binary arm, so the step
goes through the emit front-end that resolves its bridged knobs (§The non-gate
arm), and the destination comes from the scratch consumer's own
`GATE_SDK_GRAPH_ARTIFACT` — sourced in that tree — never from this tool's, which
resolved the *host* repo's config when it sourced `lib/gate.sh` at startup. The
distinction is load-bearing rather than pedantic: a host that republishes its
artifact (this repo serves `docs/check-graph.html`) would otherwise write the
scratch consumer's artifact to a path that consumer's own gate never looks at.
This is what discharges `upgrade-smoke-graph-artifact-literal`, and through
neither disposition that entry could see: it neither duplicates the default
expression nor mints an arm on a gate for one caller.

**Each phase runs against its own ref's gate binary**, and that pairing is what
makes phase 1's claim true as written. A `.gate` member dispatches to a binary
that does not travel with the kits: `native/` ships no `checks/` or `smoke/`, so
it is not a vendorable kit root and the phase-A swap never reaches it. A harness
placing one binary once therefore runs FROM's *shell* against it as well — and a
FROM tag whose own shell predates a widened binary interface reds under a pairing
that never shipped, while phase 1's message calls the tag broken. So the suite
builds **one binary per ref**: FROM's before phase 1, TO's in the same motion that
swaps the kit directories, because that swap *is* the upgrade transition. Phase
B's claim — TO's shell against TO's binary — then holds by construction rather
than by the host tree happening to be TO.

**A ref's binary comes from a detached worktree at that ref, never from the
archive its kits come from.** `native/build.rs` stamps the crate's source by
running `git ls-files` and panics where that fails, saying why: the crate builds
inside its own git checkout by construction and is never vendored. An
archive-and-build therefore dies in the build script rather than in the compiler —
a failure mode that reads as a broken tag if it is met at implementation time
instead of ruled here. The worktree is added under the scratch base and
trap-removed, so its `native/target/` is scratch as well and the host's build
output — what `check-gate-binary-fresh` judges — is untouched. The cost is not
what it looks like, and the figure is **measured rather than argued from the
manifest**: a cold `cargo build --release` of this crate from an empty target
directory took **≈4.1 s** (2026-08-14, one Linux machine, `--offline`, warm
registry cache), against a suite that vendors, installs and runs the whole battery
twice. It was ≈2.5 s immediately before the settings cohort took `serde_json`, and
the claim *those* figures replaced reasoned from an empty dependency table to *"a
few hundred milliseconds"* — wrong by an order of magnitude in a direction no
reader would check, because the crate is forty-odd modules at `opt-level = 2`,
which is what the wall clock is spent on rather than dependency resolution.
Re-measure rather than re-derive: the number moves with the module count and with
any dependency the crate takes.

**The dependency also ended this build's network independence, and the failure is
hard rather than slow.** Measured: an empty cargo home plus `--offline` now
**fails outright** (`no matching package named serde_json`), where before the
cohort that leg succeeded because an empty dependency table needs no registry at
all. The worktree build shares the host's cargo home, so a contributor who has
built the crate once is warm and unaffected; a fresh machine and every CI runner
are cold, and neither `.github/workflows/publish.yml` nor `gates.yml` provisions
a cargo cache. Whether to provision one, vendor the graph, or accept the fetch is
open and unbudgeted here — recorded rather than absorbed, because it moves a cost
from the contributor onto this suite's environment assumptions.

**A ref carrying no crate is a branch, not a special case.** Whether a binary is
needed at all comes from `csmoke_gate_descriptors` (§Consumer smoke) — the same
derivation the placement itself uses, asked one step earlier because a builder
must decide before it can name a source tree. A ref whose vendored kits carry no
`*.gate` needs none, so no worktree is added and no build runs: that is every tag
before the first one shipping `native/`, any of which a consumer may name in
`GATE_SDK_UPGRADE_FROM`. A ref that *does* dispatch and carries no crate, or whose
crate will not compile under the current toolchain, is **exit 2 under phase 1's
existing rule** — an environment or tag fact, never an upgrade finding — reported
with the ref named and the build's own stderr. The one thing it must not do is
fall back to the host's binary, which is the behavior this delta replaced wearing
a fallback's clothes.

**Both sides of the comparison are committed at their ref, and that is a second
inconsistency the same pairing closes.** Phase A archives TO, so the kits under
test were always TO's *committed* content while the binary was the host's
*working-tree* build — a dirty tree tested committed kits against an uncommitted
binary. It is named as its own consequence because a reader who sees only the
FROM half will restore the host-binary shortcut for TO on the ground that TO is
usually `HEAD`.

**What it does not cover.** The transition it proves is the *vendored kit
directories* moving FROM→TO — phase A replaces them wholesale, in tree. It never
re-runs an installer, so a consumer's **cross-version init path** is outside its
reach entirely: a green `upgrade` suite is evidence about kit contents, not about
whatever activation surface a consumer ships to deliver them. State it here
rather than leaving it inferable from the phase-A step list, since the suite's
name invites the wider reading. What left this paragraph is the **gate binary
alone**, and only for the phases the suite pairs: each runs its own ref's
artifact, so a dispatched member's FROM-vs-TO behavior is covered. The
vendored-kit-only reach still holds for the installer path.

**The gate binary is placed by the harness, and still sits outside the
determinism assertion.** That assertion covers changes under the kit roots;
phase A replaces the vendored directories in tree
and never runs an installer, so a consumer's *install* path for the artifact is
outside its reach entirely and an installed binary is neither a determinism
finding nor a determinism exemption. Widening the assertion to name it would
claim coverage this tool cannot have. Re-placing the binary at phase A does not
disturb that, and the reason is mechanical rather than argued: the scratch
consumer's `.gitignore` carries `gate_native_bin`'s path (§Consumer smoke), so
the placed artifact never enters the `git status` the assertion reads, and
nothing is exempted on its behalf. The ruling stands unchanged and is cited here,
not amended. The install path's own idempotence proof is the installer's smoke
(installer/README.md §The consumer smoke), which does re-run `init`, and the rule
that satisfies it is the manifest's: an on-disk artifact that still verifies
against the recorded digest is not rewritten (installer/README.md §The manifest).

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
- Scratch base is the existing `GATE_SDK_TMP_DIR` knob; the extracted trees, the
  per-ref worktrees and the consumer are created under it and trap-removed.

**`cargo` on `PATH` is a requirement on this suite, not on an adopter**, and it
binds only for a ref that dispatches a member to the binary. `upgrade-smoke` is a
validate-stage tool in the kit-source repo, whose contributors already need the
toolchain for `check-crate-arms` and `bin/build-native.sh`. It reaches no consumer
and does not touch `GATE_SDK_PROGRAM_FLOOR`, which bounds what a *gate rule* may
invoke — stated because a reader meeting a new `cargo` dependency will reasonably
ask whether §The port-candidate criteria's criterion 7 binds here, and it does
not.

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
byte-stable. **A `tier=commit-msg` member therefore ports with no new emitter
arm** — both hooks resolve argv through the one `command_rel` → `gate_command`
path — which the generated hook's own shell spelling reads against; the
adjudication is at §The second budget batch.

**This generator does not port, and the cause is structural rather than a sizing
judgment.** The hook bakes the **resolved** invocation argv verbatim — `env
GATE_SDK_KNOB_…=… <binary> <name>` — and resolving a knob means sourcing the
owning kit's `lib/*.sh` in a subshell and reading `declare -p`. §lib/gate.sh
rules there is exactly **one** place a knob's value is computed, the kit's shell
library; a crate-side hook emitter would have to be the second, which criterion 6
refuses. So this is not an unported emitter awaiting a cohort — it is the config
bridge's own producer, and it stays where the bridge is. `check-graph`'s
assertion D therefore keeps spawning it, which criterion 7 clears explicitly: a
rule shelling out to `bash <emitter>` clears that criterion because `bash` is on
the program floor, however unported the emitter is. The cost is real and is
recorded rather than absorbed: the spawn crosses back from the compiled substrate
into the shell one, which runs against the direction of TRAJECTORY.md §The
objectives 1, 2 and 6. It was the **first** spawn of a program other than git on
the crate's shipped gate path — never the first in the crate, whose test-only
spawn sites predate it — and it is one of several there now, but it stays the only
one of its shape: criterion 7's class-(i) wrappers spawn a program because that
program *is* the rule they assert, and this member spawns `bash` because a surface
it reads structurally stays shell. The contrast is drawn from outside the
criterion's two classes rather than inside them, and that matters — the class test
is reached **only where the criterion raises a blocker**, which an on-floor program
never does, so asking whether removing `bash` moves this gate's verdict (it does)
sorts nothing. **Ratified by the operator, 2026-08-21.**

**And the ruling is now declared where the arm that counts it can read it.** This
generator's header carries a `# no-port:` line whose cause is the paragraph above
in one sentence — criterion 6's single-producer rule, and the 2026-08-21
ratification — so §port-blockers' `--tree` arm reports it `no-port` rather than
`owed`. Until it landed, the ruling existed and the instrument that measures the
completion predicate could not see it, which is the exact misread the disposition
column was built to prevent arriving through an empty declaration set. This file
sits **squarely in the declaring class** of the substitution that arm's corpus
rule turns on — a sibling declared on a provenance ground in the same commit — so
landing the pair is what turned that substitution from a worked example into a
live one.

**The residue's disposition, ruled at build 2026-08-24, on which the debt entry
that carried it closes.** The spawn is **declared, and the declaration is what
owns it**: this member's registry entry names `bash` in the requirement element
`--needs` prints, so the criterion-7 roster answers for it on the compiled
substrate exactly as the tokenizer answered for the shell form, and one floor
filter clears it on both. Nothing new is minted and nothing else owns it. The
answer the residue was filed with — a new shell gate owning hook parity alone,
under born-native exception class (a), costed and refused here for minting a gate
name, a descriptor, a fixture pair and a SPEC section to relocate an assertion
criterion 7 already sanctions in place — is **void** rather than merely unbought,
because the operator retired class (a) on 2026-08-23 (§The port-candidate
criteria) and the refutation recorded there is that argument's own. What stays
live is the pair this section records as declined-for-now rather than refuted —
moving `--emit` into the binary, and reopening the 2026-08-21 ratification — and
this closure reopens neither.

**What the declaration does not cover, stated because it is the residue's own
edge.** `--needs` names *a program the member spawns* (§The `# graph:` manifest),
so it names `bash` and nothing the generator itself reaches for — the emitter's
own `git`, `realpath` and `awk` sit one remove further out than the
library-mediated requirements §port-blockers records as its second blind spot.
Criterion 7 is cleared either way, since all four are on the floor and its own
clause rules that a spawn target's unportedness does not reach it; what the bound
costs is that a change to the generator's requirement set surfaces in no
declaration, and review at the diff is what stands in for one. The declared set
is `bash` alone, measured rather than assumed: assertion D's two generator arms —
the second reached only where a `tier=commit-msg` member is registered — are the
member's only spawns, the three narrow modes returning before the generator is
resolved at all.

**And the measurement is what the declaration rests on, because the executed
oracle does not reach this member.** Unit test A observes a member over its own
fixture cases, both of which pass `--amend-only` here, so the observed set is
empty and the subset assertion is vacuous — the bound §The `# graph:` manifest
states for the class, with this member as its instance. What holds the spawn is
`check-graph-tree.test.sh`, which drives assertion D's both arms against a
constructed mini-consumer through the real generator; that is a behavioural
oracle and not the declaration's. So a spawn added to this member's rule reds
nothing on the declaration side, and the narrowing above was taken from a spawn
census over the gate's own resolved argv plus a read of every path reachable from
its entry point, never from a green test.

**The absent-`bash` branch was run rather than assumed, and it owes no wrapper
refusal.** With that one name scrubbed off `PATH` the member exits **2** carrying
`cannot run bash: … the check could not run; treating as failure (not clean)`,
which is §Fail-closed contract's standing backstop doing exactly its job. A
`proc::on_path` probe on top of it is **not** owed, and the ground is that the
class rule such a probe satisfies has no content on this member: `on_path` exists
so a wrapper refuses with its own message *at the shell form's own point in the
order*, and this member's shell form was itself a bash script, which on a host
with no `bash` did not run at all. There is no shell refusal to be at parity with,
so building one would be inventing the thing it was supposed to match. Recorded
so it is not re-proposed as the obvious ergonomic.

**The battery's port made this generator cheaper and left its spawn standing,
and both halves are stated so neither is rediscovered as news.** `command_rel`
calls `gate_command` for every `tier=precommit` member, so the emitter is the
loudest reader of the config bridge's cost; the per-kit batch (§lib/gate.sh)
took `--emit` from 6637 ms to 4204 ms on this tree with the emitted hook
byte-identical, which is the oracle that the batching moved no value. What did
**not** change is the ruling above or its consequence: `check-graph` keeps
spawning `bash bin/gen-pre-commit.sh` for assertion D's two arms, the pre-commit
surface and the commit-msg one, so the residue's subject survives and only its
price falls — the boundary a lead ruling of 2026-08-23 holds, keeping the spawn
outside `config-bridge-resolution-cost`'s scope. **Assertion E is not one of the
two arms**: it compares the coupling-graph artifact against an emission this
member computes in process, which is true of the compiled form and was true of
the shell form it replaced, so the generator sits in neither one's path.

**Two options were weighed against that cheaper generator and DECLINED FOR NOW
rather than refuted — ruled 2026-08-23 by the operator, and the distinction is
the whole reason this paragraph exists.** The first is the **emit-arm path**:
moving `--emit` into the binary, which the batched bridge makes arithmetically
more attractive than it was when the ratification above was taken. The second is
any **reversal of that 2026-08-21 ratification** itself. Neither was argued down
on the merits here; both were held because this unit's envelope was the battery's
dispatch. A declined option and a refuted one are not the same record, and the
refusals stated elsewhere in this section — criterion 6's single-producer rule,
the four grounds against the hook shim — are refutations that stand. These two
are open questions with a *timing* answer, so a later session may reopen either
on its merits and must not read this section's silence as either a fresh idea or
a settled no.

**The `# no-port:` declaration above closes neither of them, and the two records
do not collide.** The objection is the obvious one — the field's name says
permanent and these two options are open on their merits — and it is answered at
§port-blockers rather than here, because it generalises past this member: a cause
declares the disposition **in force under the ruling that stands**, and if an open
reopening path defeated a declaration the field would be undeclarable on any file
whatsoever, every closed ruling in this tree being reopenable by the operator. So
the header line is a reading of this section, not a fourth ruling in it. The day
either option is taken, the unit that takes it edits that line along with
everything else it moves.

**The hook's shape is ruled rather than open.** The `--run` arm makes a
two-line `exec <binary> --run --hook` shim look available; it is **refused**, and
the first of four reasons is decisive. *One*: a platform with no published
artifact would lose its whole hook — criterion 5's install model omits the
members it has no verified binary for and leaves the shell ones, and a hook whose
whole body is an exec of that binary has nothing to exec, so the branch criterion
5 exists to keep alive would die at the hook. *Two*: it moves resolution onto
every commit, where the baked hook pays zero. *Three*: it retires assertion D's
subject — the hook would stop being a projection of the manifests, so nothing
would hold a manifest edit to the hook. *Four*: it kills the `gen=manual`
round-trip, a shipped extensibility point with no replacement in the shim. So
the baked per-gate argv list is **retained**, and the install path's `--write`
step stays un-relocated: the hook's shape does not change. §gen-pre-commit's
closed refusal is what settles that, and the bootstrap's remaining work —
relocation included — is `powershell-installer-surface`'s, which
TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence names as the
tail's one remaining member.

**This is not read against TRAJECTORY.md's own two-line-shim sentence; the two
are the same ruling at two corpus states.** That paragraph states what *port
complete* means, tied to `shell-gate-tail-port`, whose completion is exactly the
`.sh` residue this refusal turns on going to zero. Once no member dispatches to
`.sh`, criterion 5's stranded-platform branch has no shell block left to lose and
the shim stops costing what it costs here. The refusal above is the correct
reading **now**, not a narrower ruling standing against a wider one already made.

**The hook bakes a ported member's resolved knob values**, because it resolves
argv through `gate_command` at *generation* time and the bridge's `env` elements
are part of that argv (§lib/gate.sh). So a kit-config edit stales the hook. That
is the established property here rather than a new hazard: the hook already
bakes a knob-derived value — the resolved `GATE_SDK_NATIVE_BIN` path sits
literally in each ported member's `run_gate` line — and the hook's currency is
held by regeneration plus `check-graph`'s byte-freshness assertion. The bridge
widens the set of knobs that stale the hook; it does not invent the property.

**Regeneration follows staging, never merely the build — and the reason is that
a derived roster reads `git ls-files` rather than the worktree.** A generator
whose input set comes from the tracked list cannot see a file that exists and is
untracked, so a unit adding one gets a green whole-tree run while the file is
invisible and a red at commit time once `git add` makes it visible and the baked
artifact is a generation behind. The ordering is therefore stage first,
regenerate second, and it applies to every generated artifact whose derivation
walks the tracked list, not to any one of them.

**Two hazards of this section compose, and the product is a red that belongs to
the probing session rather than to the tree.** Driving a ported member's
resolved argv by hand needs `GATE_SDK_NATIVE_BIN` pinned absolute, because the
default is repo-relative (§lib/gate.sh); the same pin is what `--emit` bakes
into each `run_gate` line. So a session that pins the knob to hand-drive one
member and then regenerates emits a hook carrying a machine-specific path, and
`check-graph` reds on it — correctly, since §Layout and configuration defaults
the knob relative precisely to keep the tracked hook machine-independent. Each
half is stated above; the composition is stated here because the resulting red
reads as a tree defect and is a harness artefact.

**No trigger widening is owed for it, and that is verified rather than assumed.**
`check-graph`'s `couples=` already carries `scripts/*.sh` and `kit:*.sh`, and
the hook's `staged_matches` uses bash `[[ ]]` globs in which `*` spans `/` — so
both surfaces a bridged knob's value can come from, the consumer's
`<gates-dir>/*-config.sh` and the owning kit's `lib/*.sh`, are already inside
the trigger. Adding a narrower glob would restate live coverage rather than add
any, which is why none was added. Recorded so a later reader does not read the
absence as a forgotten widening and re-attempt it.

**The alternative weighed and refused: a generated knob projection.** A shell
tool dumping every resolved array into a tracked artifact, byte-compared by a
freshness gate, is the shape this repo already uses for the hook, the graph
artifact and the docs mirror, and it would keep values out of the hook. It is
refused because the fixture runner `cd`s into each case dir, so every fixture
case would need its own generated projection — a per-fixture generated artifact
is a maintenance surface far larger than the staleness it avoids.

Emitted argv elements are quoted **deterministically**: an element made only of
shell-inert characters is emitted verbatim, and anything else — a bridged value
carrying the tab separator, or an element carrying spaces — becomes bash ANSI-C
`$'…'`. `printf %q` is deliberately not used, because its spelling varies across
bash versions and the committed hook must be byte-identical across clones for
the freshness comparison to mean anything. Both emitted hooks carry the quiet-green wrapper in their
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

**The rung reaches the gate through `gate_command`, never by interpreting the
resolved declaration path, and the reason is worth the sentence: a descriptor
interprets to success.** Running `bash` on a `.gate` file — a data file whose
whole content is comment lines — exits **0**, so once its member ported, a rung
spelled that way would not crash but *pass*, retiring the opt-in verification it
exists to perform with no diagnostic anywhere. A crash would have been the better
failure, and no gate would have caught either: this is a direct shell-out from a
non-gate script, outside every reader a port's enumeration covers.
`gate_command` resolves the invocation argv for either spelling and fails closed
on an absent binary. Its **status 1** — the member resolves nowhere — stays a
silent skip, which a consumer without this gate is entitled to; any other refusal
fails the opt-in rather than skipping it.

### build-native

The one spelling of the crate build. Invoked as
`bash gate-sdk/bin/build-native.sh [cargo-arg…]` from the repo root, it runs
`cargo build --release --manifest-path <crate>/Cargo.toml` with any trailing
arguments appended, so a per-target build supplies `--target <triple>` and still
uses one spelling rather than a second copy of the literal. The crate comes from
the existing `gate_native_crate` accessor (§Layout and configuration) — **no new
knob**, per the config-via-env convention: a knob default gains readers without
gaining spellings.

**It resolves the crate relative to cwd, never off its own location.** That is
what lets a caller running from a scratch tree — the consumer smoke's build leg —
set cwd and get that tree's crate, and it is why the contract says "from the repo
root" rather than leaving the working directory unstated.

cargo's exit code passes through unmodified and both streams pass through
untouched, so a caller capturing build output keeps working. A **successful**
cargo build can still exit 2 here, on the artifact verification below — the one
place this script substitutes its own verdict for cargo's, and it does so only to
refuse shipping a binary it has already built.

**The released artifact carries no builder path, and the remap is this script's
because this script is where every shipped build passes.** rustc records a panic
location per crate it compiles; for a *dependency* that location is absolute
under the cargo home, so an unremapped binary carries the builder's home
directory into every consumer that installs it. Measured on this tree at the
eighth cut: exactly two such paths, both under `$CARGO_HOME/registry/src/`, from
resolved dependencies rather than from the crate's own sources — cargo already
spells the local crate's paths relative, which was **probed rather than assumed**,
including under an absolute `--manifest-path`, where it stays relative. The build
therefore exports `RUSTFLAGS` carrying `--remap-path-prefix` for two prefixes,
`CARGO_HOME` (defaulted to `$HOME/.cargo`) and `HOME`: the first is the measured
leak, the second covers the same leak with a cargo home this tree cannot predict
and any future path under the builder's home. A prefix that is empty or `/`
contributes **no** flag, because remapping `/` would rewrite every absolute path
in the binary. The flags are **appended** to a caller's `RUSTFLAGS` rather than
replacing them.

**`--remap-path-prefix` rather than cargo's `trim-paths`, and the reason is the
dependency floor.** `trim-paths` stabilised in 1.81 and this crate pins
`rust-version = "1.71"`; taking it would raise the toolchain floor, which runs
against the objective that exists to collapse that floor rather than raise it
(TRAJECTORY.md). The cost accepted in exchange is that the setting lives in a
build path rather than in the manifest, which is what the verification below
exists to answer.

**The artifact is verified rather than the flag trusted.** After a successful
build, the emitted binary is matched against the consumer's own resolved
banned-pattern set — `gate_msg_pattern_files`, the same roster §check-tree-terms
and §check-commit-msg read, so the artifact is held to the tree's leak ban rather
than to a second vocabulary this tool would carry. A hit is exit 2 naming the
artifact and the matched text. This is the difference between a flag that is
*set* and a property that *holds*: a build path losing the flags, or a prefix
outside `CARGO_HOME` and `HOME` reaching the binary, is caught here rather than
in an adopter's first battery. The artifact path is derived from the crate dir,
any `--target <triple>`, and **that target's executable suffix** — the
`--target` value is named as `BN_TARGET` because two derivations now read it, the
output directory and the suffix, and the suffix is `gate_exe_suffix`'s **target**
form, never the host's (§lib/gate.sh). A cross build from Linux for a Windows
triple emits `<name>.exe` under `target/<triple>/release/`, and a host-derived
name would look for `<name>` there and report *"cargo reported success but no
artifact is at …"* — a message correct about the path and **wrong about the
cause**, diagnosing an emit-path problem where the defect was the derivation's
own. An empty `BN_TARGET` is the `--target`-less host build and takes the host
suffix by the accessor's empty-argument rule, which is what keeps a native
Windows build correct as well as a cross one. An absent artifact after a
successful build is itself exit 2 rather than a silently skipped verification.

Fail-closed (§Fail-closed contract), each **exit 2 with cause** rather than a
silent success or a skip:

- `cargo` not on `PATH` — the message names the contributor-side toolchain floor
  (context-kit/SPEC.md §bin/env-probe) instead of leaving a bare
  command-not-found to be interpreted.
- The resolved crate directory carrying no `Cargo.toml`. This message names the
  **consumer case explicitly**: a consumer receives a prebuilt, digest-verified
  binary per declared target and never the crate source (§Consumer payload), so a
  consumer tree that reaches this script has found a misuse, not a missing build.
  Saying so is what keeps a vendored tool from reading as a broken one.
- The banned-pattern set not resolving — a required tracked pattern file missing
  or unreadable. The resolver's status is read from the command substitution and
  **not** from `mapfile`, whose own status reports the array assignment and says
  nothing about the producer; reading it there would turn a fail-closed resolver
  into a skipped verification.
- The built artifact matching a banned pattern, or being absent after a
  successful build. Both are stated as assertions so "not shipped" cannot read as
  "not checked".

It is a tool, not a gate: no `# graph:` manifest, no `# install:` header, no
fixture pair — the same distinction the kits' remaining `bin/` tools carry. It is
`100755` in the index like every other `*/bin/*.sh` (§check-exec-bit).

**When to run it relative to `git add` is §check-gate-binary-fresh's rule, not
this section's.** A build made before a new crate file is staged bakes a stamp
that the tree side then cannot match; that section owns the ordering and its
ground. The pointer is here because a session investigating a stamp red reaches
this script first, and this section is where the script's own `# spec:` line
lands it.

**Why this is generic kit mechanism and vendors with the rest of gate-sdk.** It
builds *the configured crate* and carries no repo-specific path, no product
constant, and no private vocabulary; `native/` reaches it only as
`GATE_SDK_NATIVE_CRATE`'s existing default.

**The residual, stated rather than gated, and one half of it is now attested.**
Nothing stops a future session from writing a fresh longhand `cargo build
--release --manifest-path …` into a new file: no existing gate's corpus or
predicate reaches that, and inventing one would mean a banned-literal gate whose
pattern list is one repo's own build command. **The recurrence is attested**, and
naming it is what keeps this paragraph honest — §upgrade-smoke carries a second
spelling, `cargo build --release` inside a checked-out ref's crate directory. It
is deliberately left there and it is **not** a leak path: it builds a *historical
ref's* crate, which this script's current text could not govern anyway, and the
artifact is placed into a scratch consumer that **gitignores** it (§Consumer
smoke), so it never reaches a tracked tree for the leak ban to read.

**That asymmetry is the whole sufficiency argument and is worth stating once.**
gate-sdk's scratch-consumer builder ignores the placed binary; the installer's
`init` **tracks and commits** it. Only the tracked half is a leak path, and every
build reaching it — the publish workflow's per-target build, CI's, and the
consumer smoke's host build — goes through *this* script. So the property is held
in two places that do not depend on each other: this script refuses to leave a
leaking artifact behind, and `installer_smoke` runs a real `init` and a real
consumer battery over the result, where §check-tree-terms reads the committed
binary. A future second builder whose artifact reaches a tracked tree is caught by
the second; one whose artifact does not is not a leak. What remains uncovered is a
new build path that both bypasses this script **and** publishes without an `init`
— a costed filing if it is ever built, not a silent regrowth.

### port-blockers

The derived roster for the port's remaining work, at each invocation. Run
`bash gate-sdk/bin/run-gates.sh --emit port-blockers [--gates-dir <dir>] [--group | --tree]`
from the repo root: the default arm answers criterion 7 and `--group` answers
criterion 6 — both over the same registry, through the one registry walk
`registry.rs` owns — while `--tree` answers over the **tracked shell tree**
instead. It is a **bridged-arm table member** of the binary, not a `bin/` tool
(§The non-gate arm, which owns the route, the forced family and the flag
spelling); the front-end resolves its declared knobs and execs it. **Why any of the three rosters
is derived rather than written down** is §The port-candidate criteria's, criteria
7 and 6; this section owns how. The arms are **exclusive**: each replaces the
others' report rather than appending to it.

**Two corpora, and confusing them is the failure this tool is now shaped
against.** The registry arms speak for the **battery** — what `gates.list`
names — and can speak for nothing else. `--tree` speaks for the **project**, the
corpus TRAJECTORY.md §The closed rulings actually bounds when it rules that every
remaining non-test `.sh` either carries a stated cause or is deleted. The two
questions have different answers at the same moment, and a registry arm reading
*zero owed* means the battery is ported and says nothing whatever about the tree.
A session reading completion off the registry arms is reading the wrong number,
which is a mistake the tool used to make unavoidable and now does not.

**Adding an arm has twice left the arms beside it byte-unchanged, and changing
substrate left all three unchanged; each time that was run rather than claimed.**
`--group`'s addition changed no byte of the default arm's output, and `--tree`'s
addition changed no byte of **either** registry arm's — both proved by capturing
every arm across the commit, same cwd and same argv, and diffing exit codes
included. The port to the binary was held to the same oracle: all three arms'
stdout and exit status were captured from the shell tool at the commit before its
deletion and diffed against the compiled arm's, byte for byte, and the arms are
identical. What the **retirement** then moves is one row — the instrument's own —
and the trailer's *scanned* and *temporarily held* counts by one each; **the owed
count does not move**, because the file was held rather than owed. And because
the byte-identity oracle is empty over the tokenizer in a tree whose every
registered member has ported, the compiled scan was additionally held against the
shell scan's own token stream over the whole tracked shell tree, and both
registry arms against a planted still-shell registry. **That token oracle stays
re-runnable and the recovery is one command, not archaeology**: the shell scanner
was the `PB_SCAN` awk program, and `git show <deletion-commit>^:gate-sdk/bin/port-blockers.sh`
piped through `awk 'NR>=180 && NR<=356'` reproduces it verbatim, the two elided
lines being the shell quoting around it. Recorded because the extraction a session
makes to re-run this lives in scratch and dies at the next boundary, while the
line range is the only part that is not re-derivable from the file itself.
Each is a fact about that change
and **not** a standing guarantee that an arm's output never moves: the truncation
repair below moved the default arm's deliberately, and a session diffing two runs
across that commit is seeing the repair rather than a regression. What `--tree`'s
addition *did* move is the usage text, which is the new arm being documented and
not an arm's output changing.

**The default arm answers criterion 7.** For every `gates.list` member it
resolves the declaration path and reports the external programs the rule requires
beyond `GATE_SDK_PROGRAM_FLOOR`, one `<member><TAB><program><TAB><evidence>` row
each, then a trailing line counting members scanned and members carrying a
requirement it could not decide. The evidence column names where the answer came
from: a `<file>:<line>` for a shell rule the arm tokenized, the declaration path
and `(--needs)` for a compiled member that answered for itself.

**Three derivation inputs, in descending confidence.** The `command -v <prog>`
guard, already this tree's convention for announcing exactly this dependency — a
guarded program is a requirement with no inference. **Command position** — a word
at the head of a simple command, which is what excludes the attested false
positives without a per-case exception list: an element of an array literal, a
word inside a string, and an awk-internal token are all non-command-position, so
positional analysis removes them as a class. And **knob resolution for a
command-position expansion**: a `"${KNOB[@]}"` or `"$KNOB"` at the head of a
command resolves through `lib/gate.sh`'s own bridge resolver, the single place a
knob default is read anywhere, so this report cannot disagree with the value a
dispatched binary is handed.

**Those three read a shell rule; a compiled member is read from its substrate
instead, and the row is the same shape.** A `.gate` member has no rule text to
tokenize, so this arm reads the registry's own requirement declaration — the
lookup `--needs` prints, called in process rather than spawned — and maps the
three line kinds §The `# graph:` manifest specifies onto the rows it already
emits: a program
name takes the **same floor filter** a scanned command word takes, so an on-floor
program is suppressed on both substrates by one rule; a `?<TAB><knob>` takes the
**same bridge resolution** a command-position expansion takes, which is what keeps
one knob from having two resolved values in one report; and a bare `?` reaches the
**same undecidable counter** an unresolvable expansion reaches. No fourth row
shape is minted, because there is no fourth reader. **The `--group` arm does not
consume `--needs`** and is unchanged: it excludes ported members from the
partition entirely, so it has no row to fill.

**A member the registry cannot answer for is reported undecidable, never as an
empty requirement set.** In the shell form an older binary predating the arm
refused the question; in the compiled form the same branch is the **absent
member**, and it keeps the same undecidable row, because folding either into
"declares nothing" would report the member clean because the question failed —
the captured-emptiness false green of §Fail-closed contract in report form.

Two **negative** inputs keep the positive ones honest, and both are derived from
the tree rather than listed. Keyword and builtin status is asked of the
interpreter (`type -t`), a property of bash rather than a roster to maintain, and
it stays so across the port: the compiled arm asks bash in **one batched query
per run** rather than baking a roster into the crate, which would drift against
the host's own bash, and rather than making it consumer config, which a property
of the interpreter is not. The query adds no floor — `bash` is already on
`GATE_SDK_PROGRAM_FLOOR`. And `declare -F <name>` — this tree's convention for
dispatching an optional shell hook — marks a name a function however the scan
would otherwise classify it, the exact mirror of the `command -v` guard; that
harvest is a text scan and stayed one.

**The `--group` arm answers criterion 6.** It emits a **corpus-derivation
partition over the still-shell members**, groups ordered by size descending, each
member's row carrying `lines=` and the criterion columns below with its expanded
`couples=` beside it, then a trailing line counting members scanned, groups formed, members
undecidable, members already ported and excluded, members **permanently shell
and excluded**, and members **temporarily held and excluded** — closing with the
two derived totals the four exclusion classes put beyond a reader's own
subtraction, *still owed* and *takeable at this cut*. Its consumer is a **human**
session at exactly one transition — cutting the next port cohort under §The first
cohort, and the rule that selects the next — and its enabling configuration is
the one the default arm already resolves, `gate_sdk_gates_dir` for the registry
and `gate_kit_roots` for the resolve dirs, so the arm introduces no knob and no
new default to be unset anywhere. **Why this arm belongs on this tool rather than
in a new one**: criterion 6 is a question of exactly the same kind about exactly
the same corpus as criterion 7, answered today only by hand, per member, at
cohort-cut time — and the arm needs a command-position tokenizer over every
registered gate's declaration, which this tool is the only thing in the tree that
has.

**The grouping key is set-equality over two derived factors**: the pair
(kit-library call set, content-glob set), each sorted and de-duplicated, with two
members grouping together when both sets are equal. A single shared call is not
evidence of a shared derivation and the tool never treats it as one, because
**both** single-factor candidates were measured to over-select and neither
under-selects. `couples=` over-selects by criterion 4's own words — the expanded
field is deliberately **trigger**-shaped and wide on purpose. A bare
**primitive-call** key over-selects by more: `gate_kit_roots` alone has ten gate
callers spanning canon-kit, gate-sdk and `scripts/` that share no corpus whatever.
`check-shellcheck` is the worked case in both directions — it *does* call
`gate_kit_roots`, then composes four fixed subdirectory names and a `*.sh` glob on
top of it, where `bin/run-consumer-smoke.sh` composes the same call with one
`smoke/` path. Same primitive, different corpus.

- **Kit-library call set** — the command-position words the scan emits that the
  gate itself, or a kit library it can source, defines as a shell function. This
  arm is the default arm's own filter **inverted**: a name the default arm
  discards *because* it is not an external program is exactly what the key is
  made of. So there is no maintained list of "corpus primitives" anywhere, and
  the tool never classifies which library calls yield a corpus — it compares
  whole sets, which is what collapses the ten-caller over-selection above.
- **Content-glob set** — the literal extension-glob tokens (`*.sh`, `*.md`,
  `*.rs`, `*.gate`, `*.list`, …) carried by the declaration's **non-comment**
  lines. This is the factor that separates members composing the same root source
  differently, which is the `check-shellcheck` case. Reading it off non-comment
  lines alone is what keeps the `# graph:` manifest out of the key, which the
  next paragraph is the whole reason for.

**`couples=` is a printed column, never a key factor.** Each member's expanded
`couples=` prints beside it, read through the same `gate_manifest_field` +
`gate_expand_couples_var` helpers `check-graph` and the generated hook read, so
this report cannot disagree with them about what a manifest says. It is a
**cross-check**: fusing a deliberately trigger-shaped field into a content key is
the over-selection criterion 4 names, and burying it inside one fused key would
hide the disagreement the reader most needs to see. Where members share a key and
diverge in `couples=`, the divergence is a **finding for the cohort session to
adjudicate**, not something the tool resolves.

**Already-ported members are excluded and counted, not reported undecidable.** A
member resolving to a `.gate` descriptor leaves the partition entirely and is
counted in the trailing line. This is a deliberate divergence from the default
arm, where such a member prints `?` because its external-program requirement
genuinely cannot be asked; here there is no open question, because the grouping
exists to order the *remaining* corpus and a ported member is not in it.

**An exclusion counter reading zero over an empty corpus is not evidence, and
that is worth stating where the counters are.** Once every registered member has
ported, `--group` has no still-shell member to exclude, so *permanently shell* and
*temporarily held* both read zero **by construction** rather than because nothing
declares. The reading is correct and its evidential value is gone: a session
wanting to know whether anything in the tree declares a disposition asks `--tree`,
whose corpus is not empty. The general shape is worth more than the instance — a
counter over an empty corpus reports the emptiness, never the predicate — and it is
the same confusion between *nothing found* and *nothing looked at* that
§check-gate-substrate-parity's assertion G states its empty-corpus verdict against.

**Declared-permanent members leave on exactly those terms, which is what makes
the remainder mean *still owed*.** A member whose shell declaration carries
`# no-port:` (§The `# graph:` manifest) leaves the partition and increments its
own clause in the trailer, because the grouping orders the corpus this arm can
take and a member that is never going to the binary is not in it. That count is
*still owed* rather than *still shell*, the number the port track has wanted since
it started, and it is derived rather than maintained: nothing here
hardcodes a name, and the tool learns of a new permanent member the day that
member's declaration says so. **A reported split and a hardcoded exclusion both
fall out of this one input**, which is why neither was built: an exclusion list
of three names would be de-literalization's own defect and blind to the fourth,
and a split needs the same input to compute.

**Declared-held members leave the partition too — and that is why the trailer
prints two numbers instead of one.** A member whose shell declaration carries
`# port-until:` (§The `# graph:` manifest) is dropped from the grouping on the
permanent member's terms, because the grouping orders the corpus this arm can take
*now* and a member whose blocker is unlanded is not in it. That is the whole of
the field's value here: a selector's candidate list stops carrying members it must
re-adjudicate by hand at every cut. But a held member **is still owed** — being
temporarily untakeable is the entire difference between it and a permanent one —
so a fourth exclusion class silently falsifies the subtraction a reader would
otherwise do. The arm therefore reports *still owed* (held members included) and
*takeable at this cut* (held members excluded) as separate fields, rather than
leaving the second to be derived from the first.

**The failure direction differs from `# no-port:`'s, and that is what the liveness
reader exists for.** An **undeclared** hold is counted takeable — the status quo,
which a selector's own audit catches. A **stale** declaration, whose blocker landed
and whose slug moved to Done, would under-count takeable members and hide real
work. That is the direction a shape assertion cannot cover, so the slug is held to
a live queue entry by §check-gate-exemption-tasks rather than by
§check-gate-substrate-parity's assertion G alone.

**The default arm is unchanged, and that is a ruling rather than an omission.** A
permanent member keeps its criterion-7 row there. `check-crate-arms` prints
`c7=cargo`, and that row **is** part of the evidence for its own permanence (§The
port-candidate criteria, criteria 4 and 7); excluding it would delete the finding
that grounds the declaration, leaving a reader with a verdict and no oracle behind
it. The two arms answer different questions — *what external programs does this
rule require*, true and useful whatever a member's port future, versus *what
should the next cohort take*, the only question permanence bears on.

**Cheap criterion columns, so the selection rule is applicable end to end.** That
rule wants the largest set of **criteria-clearing** gates sharing one corpus
derivation, so each member's row carries the three criteria that are mechanically
derivable: **criterion 2** as `c2=pair` / `c2=no-fixture` / `c2=none`, read
through the fixture dirs §check-gate-fixture-coverage resolves and in that order,
so the report and that gate cannot disagree about whether a member carries a
pair — one **shared** resolution rather than two, taking its kit roots as a
parameter because the gate resolves absolute roots where this report's evidence
column must stay repo-relative; **criterion 3** as `c3=<tier>` from the manifest; and **criterion 7** as
`c7=`, the default arm's own verdict for that member — `clean`, the programs it
requires, or `?`. Criterion 1 is true by construction, the walk being the
registry. Criteria 4, 5 and 6 are **not** emitted: each needs judgment the tool
cannot take — a self-referential parity oracle, an aggregate binary-less
residual, whether a duplication is machine-held — and emitting a guess at them
would invite a cohort to be cut on the tool's authority instead of the session's.
No field is emitted without a named reader at a named transition, and those three
columns were considered and **removed** under that rule, since their only honest
reader would have had to disregard them.

**`lines=` clears that same rule, and it is stated here so a reader finds the
admission and the three refusals together.** The field is `wc -l` over the same
resolved declaration path the row's other columns are read from, so it cannot
disagree with the rest of its own row about which file it describes, and it sits
in the **fixed-width run ahead of `c2=`** rather than appended: `c7=` is
variable-width — `clean`, `?`, or a comma-joined program list — so a field after
it is the one column that cannot be aligned, and a cost column read down a list
is the one that must be. What separates it from criteria 4, 5 and 6 is not
importance but **exactness**: those three would have been guesses, and a reader
who cannot trust a column must disregard it, where a line count is exact. Its
reader is the session composing a **budget-arm** cut and its transition is the
cut itself (§The first cohort, and the rule that selects the next) — the reader
and transition the row's existing columns already serve, so the field adds a
column and no reader. **What it is evidence of is bounded there, not here**: the
count is a floor on a port's size and never a ranking of it, which §The first
cohort states with the two cuts that attest it.

**The field's nearest reader is a consumer, and saying so is what keeps the rule
from being bent to fit.** In this tree the takeable tier holds `check-graph`
alone, ruled out of every budget batch (§The fifth budget batch), so there is no
in-tree cut to compose until a `# port-until:` hold releases; a vendoring adopter
with a still-shell battery composes budget cuts on their own corpus, which is
what makes the column kit mechanism rather than this tree's instrument. A field
whose reader is real but not imminent is not the same thing as a field whose
reader would disregard it.

**Undecidable is reported, never guessed**, adopting §check-reads-couples'
precedent, and the ruling is stated once for the **registry arms** rather than
per arm. `--tree` is outside it and that is a property of its question rather than
an exemption: its columns are a path, a disposition and a line count, every one of
which a plain file always answers, so there is nothing it could fail to decide. An
undeclared file is `owed`, which over-counts it as work rather than losing it, and
that is the same fail-safe direction the `?` serves on the arms that need one. In
the default arm, a command-position expansion whose default cannot be resolved prints
`?`, and so does a member declaring through a `.gate` descriptor, whose rule is a
binary subcommand with no `--needs` to ask (§The `# graph:` manifest). A tool that
reported nothing for an unresolvable knob would reproduce the very false negative
the derivation exists to close — the dependency that is never spelled in the
gate's source at all. In `--group`, a still-shell member whose key is empty in
**both** factors — no kit-library call and no content glob the tool can see —
prints `?` with its `lines=` count and its declaration path and is counted; it is
never placed in a
group, and empty-keyed members are never grouped **with each other**, because
sharing an absence of evidence is not sharing a derivation. **That the count
appears on this row is a ruling, not an oversight**: the row carries no criterion
column because the arm declines to *decide* anything about a member empty in both
factors, and a line count decides nothing — it is a property of the file, not a
verdict about it — so the ground for withholding the criterion columns does not
reach it. An unkeyed member is still owed and still a candidate, and it is
adjudicated by hand, so leaving it the one row with no cost printed would
reproduce at a smaller scale the defect the column was minted against. Ruled at
authoring rather than discovered, because the branch is live code with no
instance in this tree today and a port or a rewrite would carry the omission
forward unnoticed. What the arm emits is
therefore a **decidable partition plus a counted remainder**, never a complete
partition claimed as one — which is precisely what two failed read-only sweeps
could not deliver.

**The `--tree` arm answers the directive's completion predicate, and it is the
only arm whose corpus is the tree.** It reports the port disposition of every
tracked non-test shell file as `<path><TAB><disposition><TAB>lines=<n>`, where
`<disposition>` is `owed`, `no-port` or `port-until:<slug>`, closing with a
trailer counting files scanned, files declared `no-port`, files temporarily held
and files **owed**. Owed reaching **zero** is TRAJECTORY.md's sentence made
decidable: every remaining non-test `.sh` then either carries a stated cause or is
gone. It dispatches **ahead of** the registry resolution the other two arms share,
so a tree carrying no `gates.list` still answers for its own shell — requiring a
registry in order to count files the registry does not contain is the corpus
confusion the arm exists to end.

**The corpus is derived and its exclusions are rules, not lists.** It is
`git ls-files` over tracked `*.sh`, minus the `*.test.sh` suffix — which the
directive itself names by writing *non-test* — minus the shared prune-dir set
`GATE_SDK_PRUNE_DIRS` and `GATE_SDK_PRUNE_EXTRA_DIRS` resolve, which is what
removes `gate-tests/` fixture content without naming it. That derivation is the
crate's one tracked-shell-tree rule and not a second copy of it: §check-gate-exemption-tasks'
tree arm reads the same one. **It diverges in exactly one place, and the arm
absorbs the divergence rather than the shared rule.** Where git cannot answer,
the shared rule degrades to an empty corpus, deliberately and on a monotonicity
ground its other reader states for itself; this arm **refuses** at exit 2
instead, probing for a repository before it calls, because its whole subject is
the tracked tree and a silently empty corpus would print `0 owed` — the
completion predicate — where the tool refuses. Repairing the shared rule would
break its other reader. Both are honoured, on
§check-reads-couples' ground that a substrate honouring one of an additive pair
scans a different tree than the shell for any consumer who set the other.
**Enumeration rather than a walk is correct here and this is the one place it
is**: the subject is *tracked* files, because an untracked script is no part of
what the project ships and cannot carry a reviewable declaration. The arm
introduces **no knob** — the prune set is the one `lib/gate.sh` already resolves
for every pruned walk in the tree — and a tree that is not a repository is a
**refusal**, not a silently empty corpus.

**No exclusion knob is minted for the battery runner and the bootstrap, and that
is the sharper half of the design.** The obvious spelling for *outside the battery
and the bootstrap* is a pair of path knobs, and it is refused: a knob defaulted to
one project's battery and bootstrap paths is a kit literal carrying that project's
layout — the defect §The install disposition already names — and it would need
editing every time a file moved. The directive's own predicate supplies the
mechanism instead. The bootstrap is permanently shell by a closed ruling, so it
**declares**; the battery runner is simply **owed** until its port lands. That
substitution is what makes the owed count reaching zero *be* the completion
predicate rather than approximate it.

**That substitution now has a live instance rather than only a worked example**,
and until this tree's first three declarations landed it had none — the column
counted correctly and **discriminated nothing**, a file refused by a ratified
structural ruling, a file held behind named work and a file nobody had ever looked
at all reading the same row. §gen-pre-commit's generator is the first tracked file
in this tree to sit squarely in the declaring class; the shell port oracle was the
first — and, until another file earns one, the last — to sit in the held one, and
its hold discharged when the port landed; and a consumer's measured-claim emitter
is the first refused on a provenance ground rather than a structural one. Recorded because a
reader who finds three declarations and no history reasonably assumes the field had
always been exercised, and the interval in which it was not is what the empty-set
misread above is evidence from.

**One row per file, and the columns are the ones a plain script can answer.**
`lines=` is `wc -l` over the same path the row is read from, on the same terms the
`--group` arm's own count is read and for the same reader. **No criterion column
is emitted**, and the ground is the one that removed criteria 4, 5 and 6 from
`--group`: a plain script carries no fixture pair, no tier and no `couples=`, so
`c2=`, `c3=`, `c7=` and the couples cross-check are all unanswerable for it, and a
column whose only honest reader would have to disregard it is not emitted. A
registered gate's row carries its disposition on the same terms as any other file;
**the arm does not partition gates from scripts, because the directive does not.**

**A disposition is read only from a well-formed declaration, and everything else
is `owed`.** Exactly one line carrying one of the pair, with its payload present:
a `# no-port:` whose cause is empty, a `# port-until:` naming no slug, a doubled
field and a file carrying **both** are each read as `owed`. Those are not
separate rules but one — a file that has not made a reviewable declaration has
not made one — and it is the same over-count direction absence already takes. The
triple and the header-block read it is computed from sit on the crate's universal
layer beside the corpus rule, with **two** named readers and no third: this arm,
which prints the disposition as a row's second column, and §check-gate-exemption-tasks'
tree half, which reads the held field's slug for liveness and inlined a narrower
scan of its own before the promotion. There is no fourth count because there
is no fourth disposition, and *held* is separated from *no-port* for the reason
`--group` separates *still owed* from *takeable*: a temporary hold is not a
permanent disposition, and folding the two silently falsifies the subtraction a
reader would do.

**A `# no-port:` cause asserts the disposition in force under the ruling that
stands, never an oracle about future rulings.** The objection this answers is real
and will arrive again in the same shape at every declaration: §gen-pre-commit
records two options — moving `--emit` into the binary, and reopening its
2026-08-21 ratification — as *declined for now rather than refuted*, so a field
whose name says permanent reads as overstating a question open on its merits. It
does not, and the reason generalises past that member. Every closed ruling in this
tree is reopenable by the operator; that is what an escalate-rather-than-reverse
rule is *for*. If an open reopening path defeated a `# no-port:`, the field would be
undeclarable on **any** file whatsoever, and a field with no satisfiable declaration
is a field that does not exist. The day a ruling is reopened and goes the other way,
the declaration is edited by the unit that reopened it, exactly as every other
consequence of a reversed ruling is. What the reading costs is stated at §The
`# graph:` manifest rather than absorbed: the field is weaker than its name suggests
to a first reader, so a cause names the ruling it rests on.

**And `owed` is not widened to absorb that objection instead.** The rival reading —
`owed` is correct for any file with an open port option, so the column means *no
ruling has been taken here **and** no ruling could be reopened* — is unreachable for
every file in the corpus, so it would retire the completion predicate rather than
report it. The column means *no reviewable disposition has been declared here*, and
nothing else. The remaining route, minting a queue entry for an operator-declined
option so a `# port-until:` can name it, is refused on its own ground: it puts that
option into the pickable set, which is a scope act taken by the wrong stage. Both
refusals are recorded because each is the natural next proposal from a reader who
has just met the paragraph above.

**And read only from the file's own header block — the leading run of shebang,
comment and blank lines.** This corpus contains scripts that *write* shell —
smoke scripts, installers, template authors — and a line-anywhere scan cannot tell
a declaration from a heredoc literal. The restriction is the field's own name
rather than a new rule: `# no-port:` and `# port-until:` are **header** fields, and
confining the read to the header removes the false-positive class by construction.
**Found by running the widened readers over this tree rather than by reasoning**:
the first live run reported a hold against a heredoc literal in `gate-sdk/smoke/`,
and the shell arm masked the same exposure behind the contradiction rule above,
which is why the rule is stated for the corpus and not for one reader.

**The arm reads a corpus it is outside, while the emitter that reads *it* is
inside one — and the asymmetry is the port's doing rather than a change of
rule.** A compiled arm is not a tracked shell file, so nothing about it enters
the corpus; while the instrument was shell it was tracked, non-test and outside
the prune set, so it printed itself a row. Nothing about the derivation was ever self-referential —
the arm reads headers, not behaviour, and a header the tool read about itself was
the same kind of fact as a header it reads about any other file — but this repo's
measured-claims emitter is still in the corpus and still declares, so a reader
finding a report's own consumer in it should find that recorded rather than
wonder.

**The two rows were not the same row, which is the half a reader is likeliest to
assume away, and the two files ended in different places.** They were ruled
together and they parted. The instrument was **held**, declaring `# port-until:`
against the live entry that owed its port, because it was kit mechanism: it sat in
a kit root, rode the installer payload with that root, and every adopter ran it,
so *everything portable ports* reached it with nothing to excuse it — and it is
now an arm of the binary, its hold discharged by the port rather than by a
re-reading. The emitter is **refused**, declaring `# no-port:` and staying shell,
because it is not kit mechanism at all — it is the *value* of
`CANON_KIT_MEASURED_CLAIMS_CMD`, it sits in a consumer directory the payload
assembler never packs, and it carries that project's own claim vocabulary, so
porting it would put one project's vocabulary into every adopter's binary. Held
versus refused was therefore a reading of what each file **was**, not a judgment
about how far along each one's port had got — which is why one moved and the
other did not.

**Its consumer is a human session and nothing parses it**, exactly like the two
arms beside it: the reader is a session asking whether the port is done, and the
transition is that question. The one machine consumer is a **value**, not the
format — a consumer's measured-claim emitter may read the trailer's owed count, as
this repo's does, which is why the trailer's grammar is specified above and the
rows' is not load-bearing. **No freshness gate accompanies it**, on the same
enforcement-first ground the other arms take: a gate would compare the derivation
against a stored expectation, which is the maintained roster returning by the back
door and wrong for every consumer whose tree differs.

**Two standing blind spots, recorded here as limits of the derivation rather than
left to be re-found.** Both were found by execution during the shell tail's port,
not reasoned.

- **Unregistered members.** A gate a kit ships but the consumer never registers is
  absent from `gates.list`, so neither registry arm can speak for it at all. This
  is a **corpus** limit and `--tree` closes it for the disposition question — an
  unregistered gate's declaration is a tracked file like any other. It does **not**
  close it for the *requirement* question: `--tree` reports no criterion column, so
  an unregistered member's external-program requirement is still reported by
  nothing.
- **Library-mediated requirements**, and this is the one that generalises. The
  command-position scan reads a member's own declaration text and does not follow a
  call into a kit library, so a requirement reached through a shared helper is
  invisible **even for a registered, in-corpus member** — the default arm can
  report `clean` for a gate that genuinely requires an off-floor program. That is a
  false negative of exactly the shape the repaired tokenizer above already records,
  and it is a limit of the *scan* rather than of the corpus, so no arm's walk
  closes it. Following a call into a kit library and resolving its command
  positions is a scanner widening with its own cost and its own false-positive
  surface; it is recorded here and filed as work rather than taken.

**The worked example the second blind spot was found on has since been
resolved, and saying so is what keeps the limit honest.**
`check-producer-liveness` reached `ps` only through `ek_pid_alive` in
evidence-kit's library, and was both unregistered and library-mediated — one
member attesting both limits at once. It is now a `.gate` member whose `--needs`
declares `ps` outright, so the default arm would read its requirement correctly if
it were registered. **The instance is discharged and neither limit is**: the scan
still does not follow a library call, and the next member to reach a program that
way will be reported clean by a report that cannot see it.

**The tool's own longest-standing `?` was adjudicated by a port rather than by the
tool.** `check-reads-couples` printed `c7=?` because the program at its
unresolvable command position is the gate binary — a dependency the report cannot
name because the report is about whether a member can leave shell, and this one's
answer was itself. §The sixth budget batch ported it, and the compiled form
reaches that arm in process, so the row is gone rather than resolved: the same
shape §The fourth budget batch recorded for `check-gate-binary-fresh`. Recorded
because a reader finding an old run's `?` for this member and no adjudication
would reasonably conclude the remainder had simply been ignored.

**The tokenizer rules that were bugs first, stated here because reading the
source alone reaches the wrong verdict on one of them.** They are the port's
cost centre and its risk — a character state machine over quoting,
command-substitution frames, heredocs, here-strings, double-bracket state, `case`
levels, arithmetic and array-literal skipping, with no counterpart in the crate
before it — and both repairs were carried forward by name rather than left to a
rewrite to rediscover. A **here-string** is consumed as a single redirection
operator, ahead of the heredoc branch; and **inside `[[ … ]]` only**, a `)` pops
a pushed substitution frame ahead of any case-pattern reading. Both were repaired after the scan was found abandoning most
declarations part-way and reporting the unread remainder as *clean* — a silent
under-report in both arms, and a false green in the criterion-7 roster that stood
through every cohort delivered before it.

**The double-bracket scoping on that second rule is load-bearing, and the
unscoped form was tried and caught.** Popping whenever a frame is open is the
obvious spelling and it is wrong: a genuine `case` pattern inside a command
substitution — `$( case "$x" in a) … esac )` — then has its `a)` steal the frame,
which the pre-repair code read correctly. Scoping the pop to the double-bracket
state is what makes the repair a removal rather than a trade, because inside
`[[ … ]]` a `)` is never a case-pattern close, and the in-case predicate's fault
was conflating that state with a real `case` context in the first place.
**An end-of-file balance check cannot see the difference**: an early pop drives
the frame depth to zero, the real closing `)` then lands on an empty stack and is
absorbed, and the file balances at EOF by coincidence while everything after the
`case` is lost. That is why this pair is covered behaviourally in `smoke/` by a
declaration carrying both shapes, on the ground §The `bin/`-tool contract states
for behavioural coverage — that the predicate that matters is behavioral, a
ground about the rule rather than about a file's extension, so it followed the
function into the binary. The pair is **additionally** covered by crate tests over
planted declarations, where a state machine is tested far more cheaply and far
more exhaustively than by a planted registry, and where the crate's test arm runs
in the battery through §check-crate-arms.

The here-string case is the one to state rather than leave to a reader. The
heredoc branch **does** carry a here-string guard — it declines `<<` as a heredoc
introducer when a third `<` follows — so a reader who finds that guard concludes
the defect cannot exist. The guard is **bypassed, not absent**: the generic
redirect branch below it consumed the first `<` alone, and the scan re-entered at
the second, where the guard's own condition was satisfied and the heredoc branch
fired on an operand that never recurs as a delimiter. Consuming the operator whole
and early is what makes the repair total, because an operator consumed whole
cannot be re-entered part-way. The `)` case is independent and reached through
`[[ … ]]`: the in-case predicate is true whenever the double-bracket state is set,
so a `)` closing a command substitution inside a conditional was read as a
case-pattern terminator and the restored quoting state was lost.

The repair **raises** the undecidable count, and that is it working: lines the
scan never reached carry command-position expansions it cannot resolve, and each
is now reported `?` rather than passed over. A lower count under a blind scan was
never the better number.

**The arm's argv contract.** It takes no positional arguments: the three arms
are `--group`, `--tree` and no argument at all, and `--gates-dir <dir>` names the
registry the two registry arms walk and scopes the arm's own declared-knob union.
That argument is the rule's own input rather than a redirect of resolved config,
which is the gates-dir-positional shape §The non-gate arm's distinguishing test
already rules ports unchanged. `--tree` needs no registry and **takes none**: a
documented flag that silently changed nothing would be worse than no flag.

Three of §The `bin/`-tool contract's behaviours are adopted — the arm cites that
contract for the behaviours it adopts, never for a `bin/` membership it does not
hold. `-h` / `--help` **as the first argument** prints usage on **stdout** at exit
**0**, *whatever follows it*: the help-before-arity ordering is what that contract
decides, and refusing help-plus-extra would have been a silent behaviour change on
a case already ruled. An unrecognized argument is a **refusal** — usage on stderr,
exit 2. The contract states that refusal for a leading-`-` argument; since this
arm has no positionals, a bare word is unrecognized on the same footing and is
refused identically, and so is a `--gates-dir` standing ahead of `--help`, which
is help out of the position the contract gives it. `--` is **not** adopted: it
ends option processing in favor of free-text positionals, and this arm has none to
end it in favor of. The ground is the cost that section already measures — a
session that ran a stage writer with `--help`, got `'--help' is not a lifecycle
stage` in place of usage, and went three guards deep working around a contract the
usage text would have told it did not exist. A tool with one undiscoverable mode
is that cost waiting to be paid; a tool with none was not, and one with three has
it three times over, which is why the usage text carries every arm.

**The arm's channels are `Arm::Emit`'s, and the variant was forced rather than
chosen**: a rendered document to stdout at exit 0 and a message on stderr at exit
2 is the whole of what this tool ever needs, and the compiled arm still exercises
every refusal path the shell form did — each one reaching stderr and exit 2, with
nothing left over that would want a third channel. It does not
inherit the dispatch union `Arm::Run` carries, which is correct: it dispatches
nothing.

It is an arm, not a gate: it carries no `# graph:` manifest and no fixture pair,
and the behavioural coverage stands in place of a pair for all three arms in
`smoke/` — reached **through the front-end**, which is the invocation a caller
actually makes and which no in-crate test covers. The `--tree` case plants a
corpus with each disposition, each ill-formed declaration, and one file of each
excluded class, and asserts the trailer's counts as exact **deltas** rather than
absolutes, because the surrounding tree's own shell corpus is not that
assertion's subject. **The non-repository refusal is the one leg the front-end
cannot reach**, because `bin/run-gates.sh` refuses a non-repository before it
execs; it is asserted by invoking the binary, which is a caller rather than a
second entry point, and stating that is what stops the leg being rewritten into
one that passes on git's own message instead.

**One line of one arm's output is machine-read, and the distinction is the whole
of it.** The rows of every arm are read beside a diff by the session asking the
arm's question, and nothing parses them. `--tree`'s **trailer** is different: a
consumer's measured-claim emitter may read its owed count, as this repo's does, so
that one line's grammar is an interface with a reader who breaks when it moves,
and it is specified above for that reason. Reading the count off the arm rather
than re-deriving it is derivation-first — an emitter with its own copy of the
corpus rule would be a second definition of a corpus only one of them owns.

**No freshness gate accompanies the tool, and that is enforcement-first rather
than an omission**: a gate would have to compare the derivation against a stored
expectation, which is the maintained roster re-entering by the back door, wrong
for every consumer whose configuration differs. Removing the duplication outranks
gating it, and no criterion states a roster for a freshness gate to hold.

**What that leaves uncovered, stated because it is not obvious and no gate says
it.** Where a consumer's measured-claim emitter reads the `--tree` trailer, the
emitter's resolved values are baked into the generated pre-commit hook, so the
whole tracked shell tree becomes an input to a byte-gated artifact. `check-graph`
holds that artifact fresh, but its `couples=` is a trigger over paths, and no
kit-shipped trigger can cover a corpus a *consumer's* emitter defines — which is
the seam rather than a gap to close: a kit descriptor naming one project's
directories would publish that project's layout as kit mechanism. **The port
removed a second coupling on the same seam and may not replace it**: the consumer
claim gate's `couples=` reached the shell tool through its kit-root shell glob, so
an edit to the tool re-ran the gate whose value it feeds; the implementation is
now crate source, which that field does not reach, and widening a canon-kit
descriptor to name this project's crate directory would publish this project's
layout for exactly the reason above. The gap is left covered by the
full-battery-before-every-commit rule, which is the disposition this section
already takes for the consumer-defined corpus its own uncovered set sits in. In this tree the
uncovered set is the twelve files under `installer/` and `demo/`, and the
full-battery-before-every-commit rule is what covers them, since it runs
`check-graph` unconditionally where the hook's own trigger does not. **The
regeneration cost was measured rather than argued**: over two hundred commits,
thirteen moved the corpus file set and ten of those already owed a hook
regeneration for an unrelated reason, so the marginal cost is three commits in two
hundred.

**The instrument was inside the corpus it measures, and any emitter that reads
it still is.** While it was shell, `gate-sdk/bin/port-blockers.sh` was tracked,
non-test and outside the prune set: it printed itself a row. The consequence was
not cosmetic — **the completion predicate could not reach zero while the
instrument was still shell**, and unlike the battery runner and the install
bootstrap, which the predicate disposes of by being owed and by declaring, the
instrument had to have a disposition ruled **about itself**. A `bin/` tool takes
no `.gate` descriptor, so its only port route was a non-gate arm of the binary,
and the family was forced rather than chosen: the tool needs the gates dir, the
kit roots, the prune set, the program floor, the fixture-dirs root and an
arbitrary knob besides, so it was a **bridged-arm table member** or a tool that
silently ignores every consumer override (§The non-gate arm, which owns the
route, the forced family, the minted spelling and the union sentinel). Until the
port landed, the tool's own row read `port-until:` against the live entry that
owed it, so it counted as **named work rather than unexamined work** — which is
the whole of what the declaration bought, since a held file is owed either way.

**The reflexivity was real and it was not a problem.** The arm's shell
implementation and the arm's own row left the tree in the same commit: the
measurement was not a fixed point, and its last act was to remove itself from its
own corpus. That was a property of a header read, not of a self-referential
derivation — the arm's verdict about itself was computed the same way its verdict
about any other file is, so there was no fixed point to converge on and nothing to
iterate. What the departure moved is stated exactly above: one row, and *scanned*
and *temporarily held* each falling by one, with **owed unmoved**.

### check-shellcheck

`checks/check-shellcheck.gate` (`precommit`, binary-dispatched).

Invariant: every `*.sh` directly under the consumer gates dir, each
vendored kit's `lib/`, `bin/`, `checks/`, and `templates/`, and each directory
named in `GATE_SDK_LINT_EXTRA_DIRS` passes ShellCheck at `-S warning` (the
self-lint contract). A missing `shellcheck` binary is exit 2 — a gate that
cannot run is not clean — and so is an **empty target set**, because a
derivation that selected nothing has verified nothing. The two are ordered: the
linter is probed before the targets are globbed, so a tree with neither reports
the missing linter.

A `.gate`-dispatched member is outside this corpus **with cause** — there is no
shell to lint, and `cargo clippy` at deny-warnings is the substrate equivalent
(§Meta-gate conservation for the binary substrate, which owns the reasoning).

**Its rule *is* an invocation of `shellcheck`, so it ported as the wrapper
§The port-candidate criteria's criterion 7 prescribes for a program-is-the-rule
member** (and the 2026-08-23 ruling recorded there). The compiled form spawns the
program, the program stays a **declared dependency** the payload does not carry,
and an absent one is exit 2 carrying *this member's own* refusal text rather than
the generic spawn-failure string — §Fail-closed contract owns both mechanisms and
why message parity rather than the exit code alone is what a wrapper owes. The
dependency floor is not widened by the port: a consumer without ShellCheck gets
exactly the refusal it got from the shell form, at the same point in the same
order. Parity was **run rather than argued** — both implementations over the same
corpus, same cwd, same argv, with the program present and again with PATH
scrubbed of it, byte-identical on stdout, stderr and exit code across the live
tree, both fixture cases and an empty target set.

**Criterion 4 binds, and the live-tree arm was taken undemoted — with a bound
this member has and its sibling did not.** Its corpus is every `*.sh` under a
resolve dir, so it reads gate source as content. The pre-port rule was restored
under a non-resolving name inside the resolve dir, which puts both
implementations over the **post**-descriptor corpus (§The port-candidate
criteria, criterion 4). The bound: unlike a member whose corpus is a `check-*`
glob, this one's corpus is *all* `.sh`, so the probe file is **inside the corpus
it is probing**. Both implementations were driven over the identical tree at the
identical moment and both counted the probe, so the comparison is still over the
post-descriptor corpus rather than the pre-descriptor one; what it does not claim
is that the corpus is byte-identical to the committed one, which differs by the
probe alone. Recorded because the next wrapper meets the same shape.

**What ends at the port is this tree's registration, not the kit's gate**, and
the two are recorded as separate facts because collapsing them deletes an
adopter's self-lint floor. The gate is `zero-config` and ships to every adopter,
and an adopter **cannot author a compiled gate** — `native/` ships no `checks/`
and no `smoke/`, so `gate_kit_roots` never selects it and `init` never vendors it
(§The port-candidate criteria, the default's domain). A vendoring consumer's gate
family is shell by construction, which is the corpus this gate exists for. When
*this tree's* last `.sh` leaves, its corpus here is empty and `scripts/gates.list`
drops it; the kit keeps shipping it, doing exactly the job it does today on a tree
that has shell. That is deregistration, not retirement, and it is not this port:
the tree still carries a shell library, kit `bin/` scripts and a consumer gates
directory, none of which this unit moves.

**Its own corpus narrows as this unit runs, and the narrowing is safe for a
stated reason rather than an assumed one.** The red condition is *a `.sh` under a
resolve dir that fails ShellCheck at `-S warning`*, which is monotone in the
violation set, so shrinking the corpus can only remove findings. Stated because
"a narrower corpus can only remove violations" is the first argument a narrowing
delta reaches for and it is false in general — the reds-on-empty arm stated with
the invariant above is this member's own counter-case, and it is why an emptied
target set is exit 2 rather than a clean line.

That derivation is also the answer to "does anything lint my workflows?", and on
its own the answer is no: `.github/workflows/*.yml` sits under no kit root and
under no gates dir, so it is unreached here by construction. §check-action-run-shell
is the sibling that reaches it, extracting the shell out of `run:` block scalars
and linting each at this gate's severity.

The knob **appends to** that derived set and never replaces it, so a consumer
that sets nothing keeps the shipped coverage exactly and a consumer that sets it
can only widen. It is a **whitespace-separated scalar feeding an array**, so it is
resolved in `lib/gate.sh` onto a name of its own, `GATE_LINT_EXTRA_DIRS` — the
shape `GATE_PRUNE_DIRS` and `GATE_EXEC_GLOBS` already have, for the reason
§lib/gate.sh states: a default written at a use site is invisible to the config
bridge's `declare -p`, and a consumer that never sets this knob would meet the
bridge's undeclared-knob refusal on the member's first post-port run. The
resolution splits on whitespace and does **not** pathname-expand, unlike the
inline shell form it replaces; that narrowing is shared by every knob
already on this pattern and is deliberate, a directory set being named rather
than globbed. It exists because the kit-root predicate (§lib/gate.sh) is what
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

`checks/check-gate-output.gate` (`precommit`, binary-dispatched).

**The optional `[gates-dir]` positional survives the port**, and the verdict is
recorded because the batch that ported this member deleted an argument on
another. It is consumed by the *rule* — it names the registry to read and the
first resolve dir — rather than redirecting config `gate_command` has already
resolved before the exec, which is the distinguishing test §The non-gate arm
states. No other knob this member reads is derived from it, so a caller passing
it gets one resolution rather than a mixed one; `check-kit-enum` ported the same
shape ahead of it.

Invariant: every `gates.list` member's source contains both a `: clean`
success emission and a `help:` remedy line (the static half of the output
contract). Presence is checked, not correctness — whether the clean line
actually fires is the `good/`-fixture job; whether the remedy text is accurate
is human review.

**A `.gate`-dispatched `# no-fixture:` member resolves both its corpus and its
emitter alternation off the declaration's substrate.** The static half runs only
for a member no fixture case can reach (§Output contract), and for a ported one
the descriptor it resolves to *cannot* hold the strings — the field roster is
closed and the rule lives elsewhere (§The `# graph:` manifest). So the corpus
follows the rule to the **implementation module**, at
`<GATE_SDK_NATIVE_CRATE>/src/gates/<name>.rs` where `<name>` is the gate name
less its `check-` prefix with `-`→`_`. The mapping is **derived from the name
that already identifies the gate**, by `lib/gate.sh`'s `gate_native_module`, for
the reason the descriptor carries no dispatch field: a second registry is a name
that can drift from the thing it names.

The alternation moves with it, and that half is easy to miss: the shell idiom is
`echo`/`printf` and the Rust one is `println!`/`eprintln!`/`write!`. **`printf`
is not a substring of `println`**, so a corpus that moved while the alternation
stayed would match nothing and report the member clean — a false green, not a
missed check, and precisely the vacuity moving the corpus was meant to close.

**Where the crate is absent the member is declared out of reach, and that is the
consumer's ordinary case rather than an escape hatch.** `native/` is not a kit
root and the payload vendors kit roots, so a consumer receives the descriptor
and never the module (§Meta-gate conservation for the binary substrate, *Where
that verification runs, and where it does not* — the same division already ruled
for the crate's unit tests, and stated there as making the division explicit
rather than weakening it). The gate therefore branches on the **crate manifest**,
`<GATE_SDK_NATIVE_CRATE>/Cargo.toml`, not on the module: no crate in this tree ⇒
the member is counted and **named** in the success line as out of reach, never
dropped from the accounting, because a count that quietly shrank is
indistinguishable from a member that stopped being checked. A crate that *is*
present with no module for a dispatching member is a **half-landed port and
reds** — which is the branch that matters, since upstream the crate always
exists.

**Why the manifest and not the directory.** The predicate was once
directory-presence, and that was wrong for a reason worth keeping written down:
the crate path is also where build artifacts land. `GATE_SDK_NATIVE_BIN`
defaults inside it, so *anything* that places the binary at its default —
`csmoke_place_binary` seeding a scratch consumer (§Consumer smoke), or an
adopter who hand-vendors and copies the artifact to that deliberately stable
relative path — creates the directory while delivering no crate at all. Under
directory-presence the out-of-reach branch could then never fire for exactly the
consumer it was written for, and the gate demanded a source module that tree was
never meant to receive. Directory-presence was never crate-presence; the
manifest is the file that only a crate has.

**Why not the gates module directory either.** `<crate>/src/gates/` would also
distinguish the consumer case, and it is the wrong probe in the more dangerous
direction: it conflates *crate present* with *ports landed*. A tree whose gates
module was deleted would go out of reach **silently**, converting the
half-landed-port red directly above into a green — the one verdict this branch
must never manufacture. The manifest cannot: a crate whose modules all vanished
still has its `Cargo.toml`, so that tree stays red where it belongs.

The fixture pair proves the module-grep arm in both directions; the out-of-reach
branch has its own unit test (`gate-tests/check-gate-output.test.sh`), because
both fixture cases must ship a crate to prove the arm they exist for and so
neither can stand up the tree where it is missing. That test carries the
regression case for the paragraph above — a tree holding the binary at its
default path and no manifest is out of reach — since the defect it fixes was
invisible from any tree that has a real crate, which upstream always does.

### check-gate-fail-closed

`checks/check-gate-fail-closed.gate` (`precommit`, binary-dispatched).
Invariant: every `awk`/`jq` command-substitution capture in a `check-*.sh`
gate handles its subprocess exit status — `fail_closed`, an inline
`|| { … }` guard, a captured `=$?`, or an explicit `# fail-closed-exempt:`
opt-out. Only `awk`/`jq` captures are checked: `grep`'s exit 1 on no-match is
expected, here-string captures read data already in memory, and arithmetic
`$((…))` is never matched. A parser wrapped inside a shell function is not
visible to this static scan; the opt-out covers residual false positives.

**The port narrowed one thing, and it is a report order rather than a verdict**
(§The fourth budget batch). The corpus is `<dir>/check-*.sh` expanded per
resolve dir, and the shell form's expansion ordered the files by the *invoking
locale's* collation — so the same findings printed in one order under `C` and
another under `en_US`, measured. The compiled form orders by bytes, which is
what `C` already gave. Nothing about which files are scanned or what verdict
they earn depends on it; recorded because a batch that only compares a clean
line would not have seen it, and this one compared a six-offender report.

A `.gate`-dispatched member is outside this corpus **with cause**: the defect —
branching on a captured value's emptiness when the subprocess died — is
unrepresentable once a fallible call returns a `Result` that cannot be ignored
(§Meta-gate conservation for the binary substrate). A real substrate win, not a
gap.

**The empty corpus is two different states, and this member tells them apart
rather than collapsing them.** The paragraph above blessed the corpus *shrinking*
as each member ported; what it did not anticipate is the corpus reaching **zero**,
which `shell-gate-tail-port` did. At zero the old refusal fired precisely because
the port had **succeeded** — a heuristic mistaking its objective being met for its
own failure, which is the class TRAJECTORY.md's port ruling calls engineering work
the port owes rather than an eligibility gate to be honoured. So an empty shell
corpus beside a **non-empty descriptor set** is *green with a counted zero*, the
verdict §check-gate-substrate-parity assertion G already takes for the same shape.
The invariant is vacuously true there: with no `check-*.sh` there is no `awk`/`jq`
capture that could go unchecked.

**The misconfiguration signal survives, because it is still meaningful in the
other state.** No declaration of **either** spelling under the resolved dirs is
not a finished port — it is a tree that resolved no gates directory at all — and
that stays exit 2 with a message naming both spellings. The discriminator is the
`check-*.gate` count over **the same resolved dirs** the shell glob reads, which
is what makes it faithful rather than merely convenient: before any member ported,
those dirs held `.sh` gates, so the old refusal fired only when the dir set itself
resolved to nothing. It still fires exactly there and nowhere else. No knob, no
second corpus, no new dependency. The counted zero also names the descriptor
count, so a reader can tell *the port finished* from *nothing was scanned* without
running anything else — a bare zero would have made the two indistinguishable in
the log, which is what "counted" is doing the work of here.

Neither empty state has a fixture representation — a committed case cannot be a
tree whose gates directory resolves to nothing — so both are held by a crate unit
test that drives the member over two constructed dirs, one carrying a descriptor
and one carrying neither spelling.

**Who ruled this, and why the answer is recorded rather than left to the commit
log.** It was **lead-ruled at build, 2026-08-24**. The stage session that hit the
refusal escalated it correctly — the disposition is a third gate's user-facing
semantics, which no amendment settled — and the lead ruled it under the lead's own
authority rather than relaying it to the operator. The commit that landed it
records it as *operator-ruled*; that message is wrong and is not rewritten, and
**this passage is the correct record**, on the standing rule that the owner doc is
ground truth while history answers only what happened. The label is load-bearing
rather than decorative: *operator-ruled* marks what a later session may not
reverse alone (TRAJECTORY.md, on how to read a ruling recorded there), so
inflating a lead ruling to an operator one freezes a decision that should stay
re-rulable at the lead — and it does so silently, which is why the attribution
lives beside the ruling instead of only in the amendment that carried it here.

### check-gate-fixture-coverage

`checks/check-gate-fixture-coverage.gate` (`precommit`, binary-dispatched).

**Both positionals retire with the port, and the ground is narrower than "an
argument redirects config".** The gates-dir positional was also what the tests-dir
default was *derived* from, and that default now resolves onto
`GATE_SDK_TESTS_DIR` in the kit library so the bridge can carry it
(§lib/gate.sh). A caller passing the positional would therefore move the registry
and the resolve dirs while the tests dirs stayed where the knob put them — one
argument, two resolutions, disagreeing silently. That is the arrive-a-process-too-late
shape §The non-gate arm deletes rather than reimplements, and it is why this
member's verdict differs from `check-gate-output`'s, whose positional feeds
nothing else. The tests-dir positionals go with it, redirecting that knob
directly. The fixture pair reaches the rule through the default layout instead,
which is what makes it a parity oracle for the live arm rather than for a
fixture-only second path.

Invariant: every `gates.list` member either ships a `{good,bad}/` fixture pair
(searched across the consumer tests dir, then each vendored kit's
`gate-tests/`) or carries a `# no-fixture: <reason>` header annotation. The authority set is the
registry — the gates that gate the tree — not every `check-*.sh` file. A
half-built pair is a defect regardless of any opt-out. The gate cannot
mechanically distinguish "infeasible" from "stopgap"; honesty is upheld by the
reason text.

### check-gate-assertions

`checks/check-gate-assertions.gate` (`align-only`, binary-dispatched).
Invariant: every `### <gate>` subsection in the family SPEC whose contract
enumerates its assertions is coupled to a matching `# assertion <label>:` marker
set in the gate's code, on four assertions: (A) the count-word equals the size of
the label span it introduces; (B) the heading resolves to gate code through the
registry; (C) a resolved file carrying no marker at all is the retrofit
obligation; (D) the marker label set equals the contract's label span, reported
through its `missing` and `extra` sub-branches. This catches the prose-vs-code
drift an internal count⟺span check cannot: a contract can be internally
consistent while the code grew a sixth assertion.

Discovery is first-paragraph-scoped, requires the enumeration noun
(`assertion(s)`|`axes`|`axis`|`checks`) adjacent to the count-word
(`two`…`nine`), and requires the first following parenthetical to be a
single-char `(X)` label — four filters that exclude sibling-gate mentions,
follow-on sentences, hierarchical axis/sub-rule contracts, and count-words
with non-enumeration nouns. With no spec argument the gate scans
`<gates-dir>/SPEC.md` when present plus each vendored kit's own `SPEC.md`;
each matched heading resolves to its gate source through the registry path.
The three fail-closed exits and the internal skip sentinel port
unchanged, the sentinel being no failure path at all: a `.gate`-declared member
with no crate manifest present is counted onto the clean line as *declared out of
reach*, and that segment is part of the output contract.

**Its manifest, stated because every ported sibling's section states one and
because one field of it was wrong.** `dir=bi valve=none tier=align-only`, and no
`trigger=`: the member emits into **no** generated hook, which is why criterion 3
names a real cost for it — the fixture pair, `run-gates.sh` and the align stage
are its only executed callers. `couples=` carries
`kit:SPEC.md,scripts/*.sh,kit:*.sh,native/src/gates/*.rs` **plus
`kit:checks/*.sh` and `kit:checks/*.gate`**, the two the port added. Probed at
the cut: `kit:*.sh` expands to `<kit-root>/*.sh` and no kit root holds a
top-level `.sh`, and `scripts/*.sh` matches nothing, because every gate in this
consumer's gates dir is now a descriptor. So before the correction the only
declaration path the field reached was `native/src/gates/*.rs`, while the
**walk** read `check-gate-substrate-parity`'s shell declaration on every run — a
shell gate whose edit re-fired nothing. That member has since ported, so the
missed trigger the correction closed is now reached through the `.gate` glob
instead; the finding is recorded at the corpus it was measured against, not
re-taken against today's. `kit:checks/*.sh` closes that missed
content trigger; `kit:checks/*.gate` is a **reverse trigger**, because creating
or deleting a descriptor changes which file the gate resolves and greps even
though the descriptor's own bytes are never read. The two globs that expand to
nothing stay: they are dead against *this* tree's layout, not against a
consumer's. No conservation row is added by any of it — the field already carried
`native/src/gates/*.rs`, so the member was already substrate-sensitive and
already carries its row.

**A `.gate`-declared member's markers live with its rule, in the implementation
module** — the follow-the-rule-to-the-module resolution §check-gate-output owns,
arriving here with the kit-roots cohort, because a descriptor's field roster is
closed and cannot carry them. That resolution is **one implementation shared with
§check-gate-output**, not a second copy: both reach a declaration through the
same registry path, and two copies of it could disagree about which file a member
resolves to. The marker grammar accepts either comment leader
(`#` or `//`): the leader is the substrate's and the marker is a code marker
either way. **A tree with no crate skips those members and counts them**, in the
clean line, exactly as §check-gate-output declares them out of reach — a vendored
consumer receives the descriptor and never the crate (assertion E holds the crate
outside every kit root), so a marker set that is not in the tree is not one this
gate can assert over, and reding on it would red every adopter. Crate presence is
the **manifest**, never the directory, for the reason §check-gate-output states.
The skip's executed oracle is `installer_smoke`, which runs the battery on a
freshly vendored consumer and is what found the red this branch answers.

**Four port hazards are pinned here, each a place the natural port diverges
silently rather than loudly.**

- **The lowercasing must be ASCII and byte-length preserving.** The rule matches
  on a lowercased copy of the paragraph and then slices the **original-case**
  paragraph using that copy's offsets, which is sound only because the
  lowercasing does not change the string's length. The target language's Unicode
  lowercase is **not** length-preserving — `İ` lowercases to two code points —
  so the compiled form lowercases ASCII-only. This is the sharpest
  silent-divergence hazard in the member: nothing reds, the slice simply lands in
  the wrong place past the first non-ASCII character. The offsets themselves are
  **bytes**, which is the C-locale reading of the shell form's character indices;
  an offset landing inside a multi-byte character is stepped back to the boundary
  below it, so a multi-byte paragraph faults on nothing.
- **The slice keeps one boundary character.** The count-word pattern consumes one
  boundary character on each side, and the slice deliberately starts one position
  **early** so the trailing one survives. A port slicing from the match end
  diverges on any paragraph whose enumeration noun is followed immediately by a
  parenthesis: the `(` is the boundary character the pattern ate, and losing it
  drops the span's first label and can silently demote the contract below the
  two-label arity filter.
- **The sort is byte order, and that is a stated narrowing.** The shell form's
  ordering was **locale-dependent and unpinned**; the compiled form sorts by
  byte, which is C-locale order. The two can differ
  only on a mixed-case label span under a UTF-8 locale, and no live span is
  anything but `A`-`H` or `1`-`3`. Recorded as a deliberate narrowing of a latent
  divergence rather than left as an accident of substrate.
- **The two marker grammars differ on purpose.** The extraction pattern accepts a
  multi-character label while the contract-span pattern accepts exactly one, so a
  multi-character marker can only ever surface as an *extra marker* finding. The
  port reproduces both widths rather than unifying them, because unifying them
  would turn a reported drift into an unreported one.

**Criterion 4 binds, and the contingent immunity it used to clear under is ended
deliberately. This is the finding the eighth cut was named for.** The criterion
has **two spellings in one section and they give opposite verdicts on this
member**, which is a fact about the criterion and not a reading error. Under *a
registry member's declaration path lies inside the corpus the gate scans as
content*, it **binds today, before the port, in every configuration**: eight of
the nine live enumerated contracts already resolved their markers out of
`native/src/gates/*.rs` and the ninth out of a shell gate, so the gate reads
registry members' declaration and implementation paths as content on every run,
and there is no consumer config in which the SPEC corpus stops naming gates.
Under *the gate's **own** declaration path*, it **cleared** — because the section
carried no count-word-plus-labelled-span, so the gate's own heading was filtered
out of discovery and its own bytes were never read.

**That second verdict was a contingent immunity, not a structural one, and the
contrast with the register's structural case is the point.**
`check-gate-fixture-coverage`'s immunity is a theorem: it reaches a declaration's
bytes only for a member with no fixture pair, and it must carry a pair to pass
its own rule. This member's immunity was **one sentence of prose away from
ending** — and the port was the likeliest author of that sentence, since every
ported sibling's section opens with an enumerated contract and the descriptor's
`# spec:` one-liner conventionally states one too. An immunity whose only guard
is a prohibition **this gate itself would have to enforce** is circular, and a
standing prohibition would also leave the member permanently anomalous against
every ported sibling. So the ruling is in two parts:

- **The bind is taken** under the registry-member predicate, which was already
  true and is therefore not a choice: the member joins `check-graph` and
  `check-gate-exemption-tasks` in the **no-clearing-configuration** row. This is
  also the direction §The port-candidate criteria itself takes when a verdict is
  uncertain — the conservative verdict costs a fixture widening and cannot be
  wrong in the harmful direction, while clearing wrongly ships the hole the
  criterion exists to point at.
- **The contingency is ended by making the member self-auditing.** This section
  gains its **own** enumerated contract over the four arms above and
  `gate_assertions.rs` carries the matching markers, so the gate reads its own
  module and the immunity stops being a prose accident. The precedent is
  §check-comment-tier, which audits its own declaration and whose **fixture pair,
  not the live tree, is what proves those arms** — the same sentence this member
  now inherits. The contract and the markers are two copies of one fact **held by
  the gate itself**, which is the intended coupling rather than a duplication
  defect: the set-equality arm reads every label at the comparison and the
  count arm reads the span's size, so no label can exist without a reader by
  construction. The module carries the matching authoring rule: it is the
  resolution target of its own contract, so a marker shape in its unit tests is
  **composed** and never spelled — a literal one would join the module's marker
  set and red the gate against itself, which is exactly what
  `native/src/gates/tree_terms.rs` does for the banned-shape set.

**The ordering this forced is stated, because it was not free.**
§check-gate-substrate-parity assertion A forbids a script and a descriptor
coexisting in one resolve dir, so the cross-substrate comparison necessarily ran
on the **pre-descriptor** tree. Therefore the pair widened **first**; parity was
proved over the **pair**, the only corpus inert under the port; and the enumerated
contract plus the module markers landed **with** the port, since neither can exist
before the module does. The **shared-snapshot ordering constraint binds
independently** and is discharged by none of that: this member's live corpus moves
whenever **any** of the enumerated contracts' members ports, so criterion 4
protects the oracle from the member's own port and the ordering protects every
comparison from a sibling's — independent facts, as §The port-candidate criteria
already records for `check-gate-fixture-coverage`.

**The pair widened first, and the instrument was ruled rather than left to the
build.** Measured at spec the pair stood at roughly **two of eight arms**: one
finding arm of four and one resolution arm of four. One structural cause
explained most of the darkness — both cases pass a second positional, which
short-circuits resolution to the *scripts-dir plus `.sh`* branch, so the registry
walk, the descriptor-to-module redirection and the no-crate skip were reached by
no case at all, and the **`//` marker leader**, which is the leader every live
descriptor-declared member actually uses, was covered by nothing. The `args` file
carries positionals only and cannot set a knob, so opening those arms took one of
two instruments and the choice is ruled here: a bespoke
`check-gate-assertions.test.sh` standing up a throwaway mini-consumer, on
§check-graph's own tree-test precedent, preferred to a per-case config file
because it reaches the no-crate arm — which needs a tree with **no crate
manifest**, a state a case dir inside this repository cannot have. The pair took
the rest: `good/` gained a `//`-led contract in both its plain and its indented
spelling, and three headings discovery must **exclude** — a non-enumeration noun,
a first parenthetical that is not a single-char label, and a span of fewer than
two distinct labels — none of which resolves to any gate file, so each filter is
proved by **greenness** rather than by absence. `bad/` gained the three finding
arms and the sub-branch it lacked: an extra-marker set, a count-word disagreeing
with its own span, an enumerated contract resolving to nothing, and a resolved
file carrying no marker at all. Every expectation was derived by running the
case: `run-gate-tests` reds on an `expect.txt` substring **not found**, a
zero-count red no inspection can clear.

**The parity run's verdict, recorded with the limit the register sets for a
member binding criterion 4.** Eight comparisons — both fixture cases, the live
tree with no argument, three live single-spec and scripts-dir variants, the
missing-spec fail-close and a subdirectory cwd — agreed on stdout, stderr and
exit code in every one. **Parity is proved over the pair**, the only corpus inert
under the port; the six live-tree arms are **no disagreement found on the
pre-descriptor tree**, never parity proved.

Honest residual: the marker catches editing one
side without the other, but not adding an assertion while forgetting *both*
its marker and the contract. A first paragraph that embeds the literal pattern
in example prose self-matches — the failure is loud (a false positive forcing
a reword), never a silent miss, so it is accepted.

**Its port price was `paste`, and the price is paid.** The shell rule invoked
`paste`, which is not on `GATE_SDK_PROGRAM_FLOOR` (§lib/gate.sh), so criterion 7
named it — owed port work, a dependency to be designed away or replaced, never a
permitted exclusion. *How* the requirement surfaced is worth keeping and is
recorded in that criterion's worked-example prose: the scan abandoned this
declaration before reaching the call, so the roster reported the member clean for
its whole life. The invocation was `paste -sd, -` at four sites, a comma join
over a sorted label set, which is class (ii) under criterion 7's hold-worthiness
test — the compiled rule spells the join directly and the verdict is identical
either side of the substitution. The **GNU-awk** requirement rode the same
reasoning: `match()`'s third argument was a convenience the port re-expressed in
the crate's own matcher, and the re-expression cost no new API — two of its sites
wanted the *whole* match span that §The POSIX ERE matcher's `RSTART`/`RLENGTH`
pair already reports, and the third's single-character label is read out of that
span directly. Neither was ever a hold, and this member
declared no `# port-until:` on either ground.

**The gawk floor loses this holder, and the probe that measured it was scoped to
two members — which is the honest limit, stated because the first reading of it
was not.** gate-sdk/SPEC.md carried the residual `gawk` floor as this
member's **and** §check-action-run-shell's. Probed at the cut with a `gawk
--posix` shim on `PATH`, which refuses every gawk extension: this member's rule
died on it — *match: third argument is a gawk extension*, exit 2 — and
`check-action-run-shell` ran **clean**, holding no gawk extension at all. Its own
declaration's `Requires GNU awk (3-arg match)` header was stale — the file carries
two-argument `match()` only — and was deleted at that iteration's close. The probe
covered exactly the two members this file named, so what it establishes is that
**those two** stop holding the floor — not that the floor was empty. It was not:
`check-docs-render-fidelity` was registered in `scripts/gates.list`, was still
shell, and ran GNU-awk-only `BEGINFILE`, `ENDFILE` and `ARGIND` in live program
text, so it was the floor's remaining live holder — and `shell-gate-tail-port`
retired it, the last registered member of that unit, leaving the residue empty
(site-kit/SPEC.md §check-docs-render-fidelity). What is *not* changed here is the published requirement:
`awk (GNU)` is an element of `context-kit/lib/toolfloor.sh`'s probe roster held to
docs/install.md §Requirements by `check-install-toolchain`, and narrowing a
user-facing requirement is not this port's to rule. The narrowing is filed
(`interpreter-floor-gawk-residue-empty`) rather than taken.

### check-gate-substrate-parity

`checks/check-gate-substrate-parity.gate` (`precommit`, binary-dispatched).

Holds the dispatch seam honest: a gate's implementation may move to a compiled
subcommand, but not by quietly deleting the declaration other gates read or the
record of what that move costs. Usage
`<dispatch> [gates-dir] [conservation-doc]` — both positionals survive the port,
each consumed by the rule rather than redirecting config the bridge resolved
first: the gates dir names the registry and the first resolve dir, and the
conservation doc is the rule's own second corpus. The two-arg form
steers the fixture pair onto hermetic copies of each surface. Eight assertions:
(A) declaration uniqueness; (B) subcommand parity; (C) disposition coverage;
(D) one writable home for the manifest; (E) no implementation source inside the
vendoring set; (F) one owner for the target roster; (G) port-declaration
placement; and (H) a held declaration's ground reachable in one hop.

- **assertion A — declaration uniqueness.** Each `gates.list` member resolves to
  exactly one declaration. A dir carrying both `<name>.sh` and `<name>.gate` is
  ambiguous dispatch and is red, rather than being silently settled by
  `gate_resolve`'s within-dir precedence: that precedence exists so a consumer
  can *shadow* a kit's gate, and using it to paper over a half-finished port
  would hide the state a port passes through.
- **assertion B — subcommand parity, both directions, over the kits this tree
  vendored.** The set of `.gate`
  descriptors across the resolve dirs equals the subcommand roster the binary
  carries. A descriptor naming no subcommand is a gate that cannot
  run; a subcommand with no descriptor is a gate nothing declares — unless the
  conservation section dispositions it `reference-only`, the one allowance and
  the reason it is recorded there rather than in the crate (§Meta-gate
  conservation for the binary substrate).
  **This equality is why a non-gate arm sits outside `--list` rather than inside
  it**, and the direction is worth stating forward: the binary carries arms that
  return a document instead of a verdict, and an arm that joined the roster would
  present here as a subcommand nothing declares. §The non-gate arm owns the class
  and its three properties, so the next such arm is placed correctly rather than
  discovering its placement from this assertion going red.
  **The roster half is scoped to what this tree declared, because the unscoped
  equality is unsatisfiable in any consumer that vendors a subset of the kits
  the shared binary carries** — which is every consumer once a second kit ports,
  so the gap grows by the size of each cohort rather than staying one kit's
  problem. The rule has one clause per value the owner column can take: *for a
  kit-owned subcommand, a descriptor must exist iff its owning kit is present in
  `gate_kit_roots`*; *for a **consumer-declared** subcommand — owner `-`, the
  sentinel meaning the consumer's own gates directory declares it and no kit
  ships it — a descriptor must exist iff the tree is a **publishing** tree*, the
  predicate assertion F computes, reused rather than spelled a second time and
  deliberately **source** rather than directory presence for the reason stated
  there. That predicate is `gate_authoring_tree` (§lib/gate.sh), which is where it
  lives because it acquired a second reader: §check-gate-exemption-tasks scopes
  both its arms by the same question — whether this tree authored the declaration
  it is asserting against a queue, or vendored it. A subcommand out of scope under either clause is **counted and declared
  on the clean line**. The
  equality was always meant to catch a **stranded implementation** — a subcommand
  no descriptor dispatches to, dead code or the residue of a half-finished port —
  and neither a subcommand belonging to a kit the consumer never took nor one
  declared by a gates directory the consumer does not have is that.
  **Both directions of the consumer clause carry weight, and neither is the
  obvious one.** In the publishing tree the member must be **in** scope or the
  assertion goes dark for every consumer-declared port: the tree carrying the
  crate source is the tree whose registry decides which subcommands exist, so it
  is the only tree where a stranded implementation can be created — a module and
  a registry entry landed while the descriptor is forgotten — and nothing else
  catches that, since the descriptor→subcommand direction is the other direction
  and assertion A stays quiet while a half-finished port leaves the member
  resolving to its surviving `.sh`. Ruling these members permanently out of scope
  was weighed and refused on exactly that: it would buy simplicity by ending, for
  every consumer-declared member, the one assertion this half exists for. In an
  adopter the member must be **out** of scope or the equality is unsatisfiable:
  §upgrade-smoke states the defining property of the consumer's own gates
  directory — a gate living solely there cannot appear in a vendored tree — while
  §Consumer payload has the payload carrying the prebuilt binary, so an adopter
  holds the subcommand and can never hold a descriptor for it, which is neither
  dead code nor a half-finished port.
  **Unioning the gates-directory basename into the vendored-kit name set is
  refused**, and recorded because it is the first thing a later session reaches
  for. `GATE_SDK_GATES_DIR` defaults to `scripts` for every consumer (§Layout and
  configuration), so under that union every adopter is in scope for every
  consumer-declared member and reds with a finding it cannot discharge —
  reinstating the unsatisfiable equality this scoping removed, and growing it by
  the size of each tranche.
  **Version skew across the sentinel is inherited rather than designed**, and is
  stated because it is load-bearing. Against a gate predating the sentinel, `-`
  is simply a value absent from the vendored-kit names, so the existing code
  counts it out of scope and prints the count — the same disposition this rule
  states, reached by the old code path. A newer binary in an older vendored tree
  therefore does not red, and the bounded residual below is not widened by the
  sentinel.
  Three properties are held deliberately. **The other direction stays
  unrestricted**: a descriptor the resolve dirs carry with no subcommand behind
  it is red whatever kit owns it, because a vendored descriptor is in scope by
  definition and that direction is what catches a gate that cannot run. **The
  `reference-only` allowance is untouched** and composes — an in-scope subcommand
  with no descriptor is still checked against the conservation section exactly as
  before. **The half does not go dark**: it still runs whenever the binary is
  readable, never gated on descriptor count or on the registry, which is the
  correction the reverted port paid for; out-of-scope subcommands are counted and
  printed, so an emptied scope is visible rather than silent, in the shape the
  zero-descriptor clean line already uses.
  **The owner is registry data held to executed behavior, not a self-declaration.**
  `--list` prints two tab-separated columns, `<subcommand>` and the **declaring
  root**: the owning kit's directory basename as it appears under
  `gate_kit_roots`, or `-` where the consumer's own gates directory declares the
  member and no kit ships it. The descriptor that
  would otherwise answer this is precisely what a subset vendoring lacks. The
  declaration lives in the crate's dispatch registry beside each member's walk
  roots and knobs — an entry a member cannot compile without — the same shape
  `--reads` and `--knobs` have, and a crate unit test holds it to the tree: for
  every registered subcommand the declared root carries its descriptor —
  `<owner>/checks/<name>.gate` for a kit, `<gates-dir>/<name>.gate` for the
  sentinel.
  **The owner clause is now exercised over a whole gates directory, and no
  assertion changed to get there.** The sentinel's first three members arrived
  with §The declaration cohort; §The consumer remainder cohort took the other
  ten, so `-` and the publishing-tree scope rule are proved over **thirteen**
  members — the consumer's entire gates directory — rather than over a sample of
  it. Recorded because a clause proved on three members and a clause proved on
  all of them are different amounts of evidence for the same words, and because
  the crate's owner unit test resolving every declared root to a descriptor on
  disk is what makes the second number mean anything.
  **A sentinel rather than the gates directory's basename, because a basename is
  not an identity here.** `GATE_SDK_GATES_DIR` defaults to `scripts` for *every*
  consumer, so the same string would name this repo's declaring root and every
  adopter's — the column would report a value that cannot distinguish the tree
  that owns the subcommand from the tree that merely received the binary, which
  is the exact discrimination the scope rule needs it for. A crate literal
  spelling one project's gates-directory name would also ship that project's
  layout to everyone, which §Layout and configuration rules for the target roster
  and CLAUDE.md §The provenance seam rules generally. The sentinel is
  layout-independent and needs to be, and nothing on the reading side wants the
  name: the gate resolves a consumer-declared descriptor through
  `gate_sdk_gates_dir` and already globs that directory into its descriptor set.
  **`-` rather than `?`, because the sibling arm has already spent `?` on the
  other meaning.** `--reads` prints `?` for a root the gate's author *cannot
  bound*, and its reader counts it as undecidable rather than trusting it as
  empty (§Meta-gate conservation for the binary substrate). A consumer-declared
  member's owning kit is not undecidable — it is decided, and there is none. Two
  readings that far apart must not share a spelling.
  **An introspection arm is therefore a *flag* arm, and this assertion is why.**
  Something that needs to ask the binary a question directly — a cross-substrate
  parity harness reaching a primitive no gate exposes — cannot land as a new
  subcommand: this assertion reds a subcommand no descriptor dispatches to, and the
  one allowance is the `reference-only` disposition, intended for an implementation
  held ahead of its port rather than for an arm nothing will ever dispatch. So it
  joins `--list`, `--reads`, `--knobs` and `--source-stamp` at the binary level,
  outside the roster this assertion equates. Recorded because the assertion is
  unchanged by it and the consequence is otherwise rediscovered by reddening it
  (queue-kit/SPEC.md §lib/queue.sh is the first such arm's consumer;
  §lib/declaration.sh is the second, and the shape held unchanged at the second
  use, which is what makes it a pattern rather than one harness's accommodation).
  **A second column rather than a fifth flag — and the skew that ruling was taken
  against is what this member's own port ended.** The column was chosen because
  the gate shipped in a kit and the binary in the payload, versioning
  independently: a fifth top-level flag an older binary does not recognise answers
  non-zero, which §Fail-closed contract makes exit 2, so every such consumer's
  battery would have died on a flag rather than on a finding, while a column
  degrades — an older binary prints one, the gate reads no owner, and the
  assertion falls back to the unrestricted equality. That degradation path, and
  the residual that rode with it (an adopter on a subset vendoring with a
  pre-column binary reddening until the binary is upgraded), are **retired by the
  port rather than reversed**: a compiled auditor reads the registry it is
  dispatched out of, so a binary that cannot print the owner column is a binary
  that does not carry this subcommand and cannot run the assertion at all. The
  column stays for `--list`'s other readers and the choice stays correct for the
  reason it was made; what is gone is a fallback nothing can reach. Recorded
  rather than deleted, because the *shape* of the argument — degrade a whole-roster
  read rather than refuse it — is the one a later arm serving a shell reader will
  need again.
  **Building the binary per vendoring is refused, and the refusal is recorded
  because the option reads attractive and costs a session to re-cost.** Criterion
  5's install model is closed (§The port-candidate criteria): the payload carries a
  prebuilt binary per declared *target*, built by the release and never from a
  working tree. A per-vendoring binary makes the artifact set the product of
  targets and kit subsets — every combination of kits an adopter might choose —
  which no release can enumerate and no digest roster can own, and it re-imports
  the build-time coupling the reverted port removed.
  **The predicate this shares with the registration-accounting assertion, stated
  once here so the sibling cites rather than re-derives it:** *an assertion over a
  whole-roster fact states its scope in terms of what the tree vendored.* The two
  were weighed and deliberately **not** unified — they derive scope from different
  inputs, this one from a subcommand's declaring root as the binary reports it, that
  one from the vendored kits' `checks/` directories it already reads, so a shared
  rule would be parameterised over both and become a third thing to keep true.
  **The absent-binary refusal is answered out of existence rather than retired,
  and the two-halves correction survives it in the half that still binds.** Before
  the port the gate spawned a binary it might not find, so a load-bearing one that
  was absent or non-executable had to be exit 2 — "cannot verify" and "verified
  equal" must not share an exit code — while the *roster* half ran whenever the
  binary was merely readable, descriptor count and registry both irrelevant.
  Compiled, there is no binary to be absent: the auditor is a subcommand of it, so
  that refusal has no reachable input, exactly as §check-reads-couples' did at its
  own port. What the correction was protecting is unchanged and is the half to keep
  reading forward: **the roster half must not be gated on descriptor count.** Under
  the original single guard — the whole assertion behind `descriptors > 0` — a tree
  with zero descriptors skipped both directions, which is precisely the state an
  unported tree is in and the one a stranded implementation hides in. That guard
  is still refused, and the zero-descriptor configuration is still carried in the
  bespoke test as its own case. **The roster is over subcommands
  alone**: the binary's top-level flags (`--list`, `--reads`, `--source-stamp`,
  `--knobs`) are outside it by construction, handled in the top-level dispatch
  and never entering the gate registry. Stated because the assertion's behavior
  does not change but a reader adding a further flag needs to know it is not a
  parity violation — a flag that *leaked* into `--list` would read here as a
  subcommand with no descriptor and red as a stranded implementation.
  `--knobs` is the fourth, added by the config bridge (§lib/gate.sh), and it is
  named here rather than left to be re-derived: it was written against exactly
  this paragraph's invitation to the next author. The owner column adds no fifth:
  it is roster **data** on a line the roster already carries, which is the whole
  reason it was taken instead of a flag.
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
- **assertion E — no implementation source inside the vendoring set.** The
  ruling of §Consumer payload is withholding a gate's predicate, and what a
  consumer receives is exactly the kit roots (`gate_kit_roots`), so the ruling
  is violated structurally and is checked structurally in two halves. **No
  implementation sibling:** for every `<name>.gate` the resolve dirs declare, no
  file named `<name>.<ext>` exists anywhere under any kit root — the natural
  mistake of putting a ported gate's implementation beside its descriptor, where
  it vendors. **The crate root is outside every kit root:**
  `GATE_SDK_NATIVE_CRATE` resolves to no path under any `gate_kit_roots` member.
  That half is the non-vacuous one in an unported tree, and it reds on the single
  edit that would silently undo the whole ruling — relocating the crate to give
  it a ride into the payload. It is folded into this gate rather than shipped as
  its own because the descriptor set it reads is the one this gate already
  derives, and a separate gate would duplicate that derivation to add nothing.

  Three calibrations, each drawing a boundary rather than trimming for
  convenience. The extension test is **extension-agnostic**: any extension but
  the two spoken for reds, because a roster of implementation languages is a
  maintained set that rots and would contradict assertion D's own
  language-agnosticism. `.gate` is spoken for by the descriptor itself and `.sh`
  by assertion A, which owns within-dir ambiguity and by §Layout and
  configuration sanctions a consumer's shell shadow of a ported gate. And an
  **extensionless** name is deliberately out of reach: a built artifact named
  after its gate is an artifact-placement question this section does not own, and
  an assertion that reds on it would prejudge one. **The sibling half's corpus is
  the kit roots, so it does not reach a file dropped beside a
  *consumer-declared* descriptor — and that is its scope rather than a hole**: the
  half enforces §Consumer payload's ruling, whose subject is what a consumer
  *receives*, and a file in this repo's own gates directory vendors nowhere. The
  honest residual, stated
  rather than discovered: with no descriptors declared the sibling half scans
  nothing and says so in its clean line — an unported tree has no ported gate's
  source to misplace, and the crate half stays live throughout.
- **assertion F — one owner for the target roster.** §Consumer payload rules the
  roster the single surface asserting platform support, and prose cannot hold
  *no second spelling* — so the three ways that ownership breaks are checked
  here rather than at review. **The roster is well formed:** every live line of
  `GATE_SDK_NATIVE_TARGETS_FILE` is a `<arch>-<vendor>-<os>[-<env>]` triple and
  the file is non-empty, because a roster asserting no platform support cannot be
  the surface that asserts it. **The build matrix is roster-derived:** every value
  in a `matrix:` declaration of `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` is a GitHub
  expression, never a literal — a hand-written platform there is a second
  spelling of the support commitment. The runner mapping is untouched by this and
  deliberately so: it is a *runner selection*, not a support declaration, and it
  is the one place a platform name may appear in the workflow. **Each digest has
  one producer:** a step *computes* a digest when it invokes `sha256sum` without
  `-c`; no job may compute more than one, and a job that downloads a run artifact
  and uploads none may compute none at all. Verification (`sha256sum -c`) is
  unrestricted — it is what `pack-installer.sh` and the installer do, and it is
  the opposite of the failure being prevented.

  Two absences are reported rather than red, and each for a stated reason. A
  **missing publish workflow** is not a publish path to audit: a consumer whose
  release rides elsewhere points the knob at it, and one with no release at all
  has nothing here to get wrong. A **missing roster** is red only in a
  **publishing** tree that also **dispatches** — both conditions, and each one
  removes a different false red. Dispatch, because a *declaration* needs no
  prebuilt binary and a descriptor-count trigger would red every vendored tree
  the moment the first cohort's descriptors ship. Publishing, because declaring
  platform support is the act of the tree that *builds and publishes* the
  artifact: a consumer receives the kit roots and never the crate, so its roster
  is absent **by construction** (§Consumer payload) while it may perfectly well
  dispatch to a binary it was given. The publishing test is *the crate's tracked
  source is here* — step 1 of §check-gate-binary-fresh's source stamp, reused
  rather than spelled a second time, and deliberately **source** rather than
  directory presence, so build output or a placed artifact under the crate root
  cannot make a consumer look like a publisher. Together they keep the assertion
  from being either vacuous in this repo (which has a roster, registers both
  ported members, and is checked in full) or hostile to a consumer that has no
  crate.
- **assertion G — port-declaration placement.** Five clauses over the declaration
  set assertion A already derives, which resolves every member to exactly one
  declaration and knows which spelling it got — the entire input, which is why
  this folds in here rather than shipping as its own gate, on the precedent
  assertion E states for itself. **A `.gate` descriptor carries no `# no-port:`
  line and no `# port-until:` line**, §The `# graph:` manifest's placement rule for
  both fields, and the one failure the
  mechanism can actually produce: a port landing a descriptor from a declaration
  that carried the field, forward-copying it with the `# graph:`/`# install:`/
  `# spec:` lines that *are* copied verbatim. **A `# no-port:` line on a shell
  declaration carries a non-empty cause, and a declaration carries at most one** —
  the shape §check-install-disposition already holds for `# install:`, reached
  here with *at most* since the field is optional — and **a `# port-until:` line
  carries a non-empty slug on those same terms**. The fifth is the clause neither
  field owns alone: **no declaration carries both**, because permanent and
  temporarily-held are contradictory verdicts about the same member, and a
  declaration asserting both makes §port-blockers' two exclusion counts overlap.
  The subject fits this gate:
  where a declaration field may live by spelling is the dispatch seam's own
  partition, the question assertion D answers for the
  implementation-versus-declaration axis.

  **Presence is deliberately not asserted for either field, and the asymmetry is
  what makes that safe.** No clause demands that a permanently-shell member
  declare, because permanence is a ruling in prose and a gate deriving which
  members hold one would have to parse SPEC argument text; the same holds for a
  blocker. The error direction is what bounds it: an
  **undeclared** permanent or held member is counted as takeable, which is the
  status-quo over-count for that one member and nothing worse, while both fields
  can only ever
  *shrink* the reported takeable set — so the mechanism fails toward today's state
  rather than toward an under-count. Enforcement-first is not waived; it ranks a
  gate above discipline *where a gate is available*, and the available gate is
  this shape assertion, which ships.

  **What this assertion does not cover is the held field's stale direction**, where
  the blocker landed and the slug moved to Done. That is slug **liveness**, not
  shape, and it lives in §check-gate-exemption-tasks — the split the two fields
  already draw: a field's spelling-domain is the dispatch seam's own partition,
  while a claim about the queue belongs with the gate that already reads the queue.
  Landing it here instead would give this gate a queue-file coupling it
  deliberately has none of, it being the auditor of the dispatch seam.

  **Its corpus narrows to empty in this tree, and the verdict there is
  green-with-a-counted-zero.** Within **this assertion's** corpus — the declaration
  set assertion A derives — both fields reach only the `<name>.sh` spelling, so once
  no registered member declares in that spelling the clause set has nothing to range
  over. That is a statement about G's corpus and **not** about the fields' domain,
  which §The `# graph:` manifest widened to any tracked script: a plain script
  carrying either field is read by `--tree` and, for a slug, by
  §check-gate-exemption-tasks, and is invisible here by design. That is a **corpus narrowing**, one of the three non-monotone shapes
  §The causal-completeness check point 5 names, and the red condition is stated so
  it stays on the right side of it: *a `<name>.sh` declaration whose field is
  malformed or whose `# spec:` section does not state the hold* — never *no
  declaration found*. The counted zero is on the clean line for the same reason
  assertion H's grounded count is, and the empty case is carried explicitly in the
  bespoke test rather than left to arrive with the last port.

  The gate was shell under the since-retired exception class (a) when this
  assertion landed, so widening it raised no substrate question, and the
  assertion adds no member to the conservation table: the corpus is the
  declaration set it already walks.
- **assertion H — a held declaration's ground is reachable in one hop.** For every
  declaration carrying `# port-until:`, the SPEC section its own `# spec:` header
  field points at names the field. §The port-candidate criteria rules that a held
  member's ground lives in its own section and §The `# graph:` manifest rests the
  slug-only payload on it; until this assertion that rule was discipline, and two
  of the five declarations live when it landed failed it.

  **Resolution is the declaration's own pointer, not a derivation.** Every
  declaration carries a `# spec: <path> §<section>` header field, and
  canon-kit/SPEC.md §check-spec-pointer already holds that pointer to a tracked
  file and an existing heading — so this assertion resolves *nothing*: it opens
  the section that field names and reads it. That is the literal shape of the
  property, a reader reaching the ground from the declaration in one hop, and it
  costs no second heading-matching implementation. It also handles the two heading
  levels in play without a special case (`### <gate>` here, `## <gate>` in
  site-kit/SPEC.md), because the pointer names the heading **text** and the
  extraction runs to the next heading at the same or shallower level.

  **Red conditions, enumerated.** A declaration carrying `# port-until:` and **no
  `# spec:` field naming a section** — either the field is absent, or it carries a
  path with no `§<heading>` fragment — is red naming the declaration: the ground is
  unreachable from it, whatever else is true. This is not `check-spec-pointer`'s
  job, which holds a *present* pointer's resolution and asserts no pointer's
  presence. A declaration whose named section body contains no `port-until` token
  is red naming the section: the hop lands somewhere that does not discuss the
  hold, which is also where a heading the file lacks arrives, its body being
  empty. And an **unreadable target file** at the resolved path is fail-closed on
  §Fail-closed contract's ordinary terms — a gate that cannot read its corpus is
  not clean.

  **Not asserted, deliberately:** that a member *without* the field has no hold.
  That is assertion G's presence question, refused there for reasons this does not
  disturb, and the error direction of not asserting it is the status-quo
  over-count.

  **The corpus and the coupling.** The subject set is the declarations assertion A
  already resolves; the second corpus is the SPEC files their pointers name. The
  manifest's `couples=` therefore gains `kit:SPEC.md`, so an edit to any vendored
  kit's SPEC re-runs the gate — the correct coupling, since deleting a paragraph
  is exactly how this assertion goes from green to red, and without it the most
  likely red would arrive a tier late. The existing `gate-sdk/SPEC.md` literal is
  **kept rather than folded into the new token**: it is assertion C's
  conservation-doc coupling, and a consumer whose `GATE_SDK_KIT_DIRS` does not
  name gate-sdk would otherwise lose it silently. Two couples for two assertions,
  stated so the redundancy does not read as an oversight. The widening's direction
  is *wider*, so the monotonicity argument below does not bind on it; what it costs
  is one section read per held declaration, bounded by the held tier.

  **Why it folds in here rather than shipping as its own gate.** Assertion C is
  the precedent and it is exact: C asserts that a *SPEC document records a
  disposition* for every substrate-sensitive member — the same prose-placement
  claim over the same declaration set, in the same gate, read from the same
  two-positional usage. The refusal recorded at §check-gate-exemption-tasks, that
  slug liveness would give this gate a **queue-file** coupling it deliberately has
  none of, does not reach a SPEC coupling it already has. And the gate was shell
  under the since-retired exception class (a) when this widening landed, so it
  raised no substrate question and adds no conservation row either.

  **Its honest limit, stated rather than discovered.** The assertion holds
  *reachability*, not truth: a section could name the field and say nothing
  useful, and no gate can rule whether a ground is a good one. §When a gate earns
  its place bars a trivially-true proxy, and what clears this one is a measurement
  rather than an argument — two of the five live declarations failed it at the cut
  that landed it, so it is a real drift axis over a real corpus and not a
  heading-presence check. What stays human is the irreducibly semantic judgment
  alone. The count of held declarations whose ground was verified is on the clean
  line for that reason: a zero there in a tree that declares holds is the
  vacuous-pass tell, and its reader is the session choosing the next cohort cut.

  **It must not acquire a non-zero floor**, stated because the vacuity instinct
  would add one: a tree with no held member has nothing to ground, and a floor
  would red every consumer that never declared a hold. The anti-vacuity signal is
  the emitted count, not a refusal.

**It ported, and the argument it had stood on is the one the operator retired.**
The reading it held was *a gate that audits the port is not a gate the port may
consume, or assertion B would be checking a roster through the very binary whose
roster is in question* — born-native exception class (a), retired 2026-08-23 with
its refutation recorded at §Meta-gate conservation for the binary substrate: the
shell auditor already trusted `--list`, so the spawn never bought independence in
the first place; an absent binary was exit 2 under the §Fail-closed contract; and
a **stale** binary — the only state where an in-process and a spawned answer could
differ — is §check-gate-binary-fresh's red and never this member's. The compiled
form therefore **reaches the registry in process and the spawn is deleted**, the
shape §check-reads-couples took at its own port, so this member's `c7` row is gone
rather than answered. What it does spawn is `git`, once, for assertion F's
publishing test, and it declares that (§The `# graph:` manifest, `--needs`);
`port-blockers` never saw it because the call sits in `gate_authoring_tree` rather
than in the gate's own text, which is the shared-library blind spot
`port-oracle-corpus-narrower-than-the-directive` owns.

Its coverage is split across three oracles because the configurations cannot all
live in one fixture pair — a pair is one invocation each — and naming where each
is proved is what keeps the split from reading as a hole. The
`good/`+`bad/` pair covers **the declaration set**: assertion A's ambiguous
dispatch, the descriptor→subcommand direction, and every one of assertion G's
clauses — a descriptor carrying either field, a bare one with no payload, a
declaration carrying two of one field, and a declaration carrying one of each. It
covers **assertion H** in the same invocations: a held declaration whose
pointed-at section names the field clears, one whose section does not reds, and
one with no `# spec:` header field at all reds. The SPEC surface lives inside the
case tree and each pointer is case-relative, so the pair proves the resolution
rather than the live tree's accident of already being green. It also covers
assertions C, D, E and F end to end, each over the case's own hermetic surfaces.

The bespoke `gate-tests/check-gate-substrate-parity.test.sh` holds the
declaration configurations one pair cannot reach, each a sandbox rather than a
live tree: **no descriptors at all** — the post-revert tree, where the roster half
is the only live half and a descriptor-count guard would blank it out;
**descriptors present, none dispatching, and no roster** — every vendored tree
once a cohort's descriptors ship, where assertion F's missing-roster arm must stay
quiet; **a consumer dispatching to a placed binary with no crate**, which is also
assertion G's **empty shell-declaration corpus** reported as a counted zero rather
than as a red for finding none; and **the publishing counterpart**, where the same
absent roster reds because declaring platform support is the act of the tree that
builds the artifact. The last two are that predicate's own boundary, and a
too-loose one passes the `good/` case and reds only there.

**Assertion B's roster matrix lives in the crate's own unit tests, and the port is
what moved it there.** A roster is not name-addressable, so where
§check-reads-couples' cases could name a real registry member and let the
substrate answer, this member's could not: the configurations that matter are
*whole rosters* — a subset vendoring, its near miss where the in-scope kit is the
one missing a descriptor, the consumer sentinel out of scope in an adopter and in
scope in the publishing tree, the descriptor→subcommand direction staying
unrestricted under the scoped path, and the reference-only allowance. Every one of
them was a stub binary printing a manufactured `--list`; with the spawn deleted a
stub tests a provider the gate does not have. So the comparison is a
**pure function** over the descriptor set, the roster, the vendored kit names, the
publishing flag and the conservation section, and those six configurations are
driven against it directly. Nothing weakens in timing: the crate's tests run at
commit through §check-crate-arms exactly as the bespoke test runs through the
fixture-runner battery, and a roster held as a value is *more* legible than one
held as a process. Recorded rather than left to be rediscovered, because a reader
finding the old stub-binary cases gone and no adjudication would read a coverage
loss where there is a relocation.

**Why those configurations are held in fixtures rather than assigned to live
trees.** A coverage claim naming a tree is only as durable as that tree's
configuration, and a landing cohort changes configurations: this repo registers
every ported member, and the smoke consumer carries their descriptors. A claim
that quietly stops being true is worse than none, so each configuration lives
where nothing but an edit to the test can change it.

**Assertion E's couples were widened for it, and the widening is the finding.**
The manifest carried `kit:checks/*.sh,kit:checks/*.gate`, which covers neither
subject assertion E reads: not a non-shell file dropped beside a descriptor, and
not the crate root at all. An assertion that never re-fires is an assertion that
never runs, so the couple became `kit:checks/*` — the one location the sibling
mistake lands — plus `native/*` for the crate root a relocation stages away
from. What stays uncoupled is the recursive sweep of every kit root: coupling a
whole kit root recursively would fire this gate on nearly every commit to buy a
trigger for a case the full battery already reaches, and the sweep reads file
*names*, never file content, so there is no content surface to bind.

It is registered in gate-sdk's consumer-smoke install: it needs no consumer
config, and a vendored tree is exactly where a descriptor with nothing behind it
would land.

### check-gate-binary-fresh

`checks/check-gate-binary-fresh.gate` (`precommit`, binary-dispatched).

Invariant: **whenever a `.gate` descriptor makes the binary load-bearing, the
binary was built from the source now in the tree.** Usage
`<dispatch> [gates-dir] [tree-stamp-file]` — both positionals survive the port,
each consumed by the rule rather than redirecting config the bridge resolved
first: the gates dir names the registry and the first resolve dir, and the
stamp file is the tree side of the comparison itself.

**The tree-side derivation is runtime crate code, which it had never been.** The
crate carried the stamp as a **build-time-baked constant** plus a test-only
comparison, so a `gate_native_source_stamp` counterpart had to be written for
this port — the same three git invocations `native/build.rs` runs, now with a
runtime caller. The two are one algorithm and a divergence would make the
freshness verdict meaningless, so the module's own unit test asserts the runtime
derivation equals the constant the build baked. That test is also where the
git-based arm is exercised at all: **both** fixture cases pass an explicit stamp
file, so the derivation the gate exists for has no committed case, and saying so
here is what keeps a green pair from reading as coverage of it.

**A bespoke case for this member cannot vary the binary through the dispatch.**
Its subject *is* `GATE_SDK_NATIVE_BIN`, which `gate_command` also resolves the
dispatch through, so pointing that knob at a missing binary makes the dispatcher
refuse before the rule runs. `gate-tests/check-gate-binary-fresh.test.sh`
therefore resolves the argv through `gate_command` — never a declaration path —
and substitutes the one bridged element the case varies. The fixture pair needs
no such handling: `run-gate-tests` resolves the dispatch binary once at the
invoker's root, outside any case's config.

**What makes the binary load-bearing — the predicate both binary meta-gates
share.** A `.gate` file on disk is a **declaration**. A **registered member
resolving to** a `.gate` file is a **dispatch**. Only the second makes the binary
load-bearing, and only the second can run stale. This is not a new rule but this
section's own zero-descriptor reasoning applied at the right granularity: the
clean report below is grounded on *no gate dispatches to the binary*, and reading
"a descriptor exists" as "a gate dispatches" is what was wrong. A member
`gates.list` does not carry — never registered, or registered and then commented
out by `init`'s `# omitted:` record, which the runner already strips from the live
set — dispatches to nothing.

It needs **no new field, no marker file and no new knob**: the omission record
`init` already writes removes the member from the live registry, so one predicate
covers the omit path, the unregistered-in-a-vendored-tree case, and the
fully-registered case, with nothing to keep in sync. The registry is read at one
decision — is any live member's declaration a `.gate` — and both counts are named
in every report, because *no descriptors* and *descriptors nothing dispatches to*
are different tree states that must not share a report line. An **absent**
registry is exit 2 rather than a clean report: the live member set is what the
verdict turns on, so a tree that cannot produce one is "cannot verify".

**Getting this predicate wrong fails silently in both directions, which is why it
is stated rather than left to the code.** Too loose and a stale binary passes
unnoticed; too tight and every tree that vendors a descriptor it does not
register reds — including a real adopter on `init`'s own omit path, whose battery
would exit 2 on every run and defeat that design's promise that a re-run on a
machine that has since gained a hasher converts the member back into a live one
with no hand edit.

**This gate is not a consumer gate: a consumer tree does not verify a build it
did not make.** Its subject is the crate the binary was built from, and the
tree-side stamp is `git -C <crate> ls-files` over that crate's tracked source.
§Consumer payload keeps the crate outside every kit root, so it is never
vendored and **no consumer tree has one** — the gate's subject does not exist
there, and it exits 2 ("cannot verify") however the binary arrived. That is the
right verdict rather than a defect to patch: §Fixture-pair discipline already
rules that a consumer's question "is not *did my build come out right* — that
question does not exist for a consumer", and the consumer's own oracle is the
digest (which artifact arrived) plus the fixture pair (what it does). So the gate
runs in a **building** tree, this repo and CI, and is deliberately absent from
the consumer smoke's scratch registry, where the accounting derives its exemption
every run rather than accepting a written one (§Consumer smoke).

§check-gate-substrate-parity assertion B diffs the descriptor set against the
binary's `--list` roster — set membership only, never the binary's *content*
against the source it was built from. `gate_command` dispatches a
`.gate`-declared member straight to the prebuilt binary with no rebuild and no
freshness check, so editing a gate's Rust source, skipping the rebuild, and
committing runs the descriptor-named gate **against the stale binary**, where it
passes on the old implementation. A gate reporting clean on code that is not what
is committed is the vacuous green the whole battery exists to refuse.

**git is the hasher, and that is the ruling the design turns on.** Hashing is the
classic place two implementations drift — a Rust digest and a shell digest over
"the same" input set are one canonicalization disagreement away from a permanent
false red. git is already the sole runtime dependency the trajectory commits to,
shelled out rather than embedded, and content-addressed hashing is its native
operation. So both sides call **the same tool with the same arguments**: one
algorithm, rather than two implementations of one algorithm.

**One ground this ruling used to carry has been deleted as false, and deleting it
is not reopening the ruling.** The text argued additionally that a digest crate
*"puts a first dependency into the artifact every adopter's machine carries"*.
That conflates two different dependency sets, and the operator corrected it on
2026-08-14: the constraint is the **adopter's** — their machine requires only git
and the pre-compiled binaries (TRAJECTORY.md §The objectives, objective 4) — and
it says nothing about the crate's build graph, which no adopter ever receives,
resolves or compiles. The drift argument above is the ruling's ground and is
untouched by the correction: two implementations of one hash is the risk, whoever
wrote either.

**The source stamp.** git's content identity for the crate's tracked source set,
computed by three invocations that are identical on both sides — and, since
§check-crate-arms ported onto the same value to key its cache, by **one** runtime
helper rather than one per reader (`fresh::source_stamp`), so a second reader could
not become a second algorithm:

1. `git -C <crate> ls-files` — the input set is **derived, never maintained**.
   Neither side carries a roster of which files matter, so neither can carry a
   stale one. The honest cost is named rather than discovered: an edit to a
   non-code tracked file such as the target roster changes the stamp and asks for
   a rebuild although it changes no code. Rare, harmless, and strictly better than
   a hand-maintained roster that silently omits the file that mattered. The
   ordering is `ls-files`' own index order, which is byte-sorted by path — taken
   rather than re-derived, because a `sort` on either side would be a
   locale-sensitive second canonicalization of exactly the kind this design
   refuses.
2. `git -C <crate> hash-object -- <path>…` — each input's blob hash over the
   **worktree** content, not the index, because the worktree is what `cargo`
   compiled and an index-derived stamp would go green on a build made from
   unstaged edits.
3. Those hashes joined with their paths into one listing, hashed again by
   `git -C <crate> hash-object --stdin`.

**Its honest limit, stated rather than discovered:** an **untracked** new source
file is invisible to `git ls-files`, so it is compiled but not stamped. This is
bounded by the same fact that makes the stamp worth having — an untracked file is
not in the commit either, so the state the gate governs is exactly the state the
stamp describes. A file added and staged in the same commit is tracked at the
gate's read and is covered — **provided the build ran after the `git add`**. The
producer reads `ls-files` too, so building while a new source file is still
untracked bakes a stamp computed without it; staging the file then moves the tree
side and not the baked one, and the commit reds on a rebuild that has already
run. A commit introducing a crate file stages first and builds second.

**A partial-path commit cannot split a port, and it is the same `ls-files` read
that says so.** `git commit -- <paths>` commits against a *temporary* index of
`HEAD` plus the named paths, so the input set step 1 derives shrinks to that set:
a commit naming only some of a cohort's crate files recomputes the tree-side
stamp without the rest, and no binary built from the whole worktree can match it.
Porting a cohort is therefore **atomic by construction** — the crate modules, the
registry edit, the descriptors and the deleted shell gates land in one commit or
this gate reds. That is a mechanism rather than a convention, so no sequencing
rule has to carry it and no session has to remember it; what a session does need
is to read it here instead of re-deriving it from two refused commits.

**The two sides.** The producer is `native/build.rs`, which bakes the stamp in as
a compile-time constant and is re-run by `cargo:rerun-if-changed=` on exactly the
two events that can change it: each tracked input's own path, for a content edit;
and the git index, because the index is the only way a file enters or leaves
`git ls-files`. Deliberately no directory sweep — one would drag `target/` in and
rebuild the crate against its own output. A build that cannot compute the stamp
**fails the build** rather than emitting a sentinel: a binary with no stamp is
exactly the artifact no freshness oracle can hold, and the crate builds inside its
own git checkout by construction (§Consumer payload keeps it out of every kit
root, so it is never vendored). The tree side is `gate_native_source_stamp`
(§lib/gate.sh), the one shell home of the same three invocations.

**The comparison.** With ≥1 descriptor the gate runs `<binary> --source-stamp`
(§Porting a gate to the binary substrate), computes the tree-side stamp, and reds
on a mismatch. The failure text names both stamps, the descriptors that dispatch
to the binary, and the rebuild command, so the remedy is in the output rather than
in a reader's memory.

**Running the crate's tests does not discharge the rebuild, and this is measured
rather than reasoned.** `cargo test` compiles its own harness — a different
artifact from the binary `GATE_SDK_NATIVE_BIN` names, which is what this gate
reads. A full `cargo test --release` can pass every test in the crate, including
the one proving both substrates compute the source stamp identically, and leave
this gate red on the stamp baked into a stale binary. So the obligation is
`bash gate-sdk/bin/build-native.sh` (§build-native) as **its own named step**: a
tree with a live descriptor must build the binary before it commits, and a
contributor routine that states a `cargo test` line has not stated that step. Recorded here because the omission is
invisible from a green test run — the exact shape of vacuous green the section
above exists to refuse.

**Trigger coupling: nothing dispatching is a clean report, not a skipped
assertion.** No gate dispatches to the binary, so nothing can run stale — the same
coupling shape assertion F uses for the target roster, reused rather than
re-invented, and since this unit both arms read the same predicate rather than
merely the same shape: a consumer with no crate is not a consumer with a defect.
With ≥1 **dispatching** member an absent or non-executable binary is **exit 2**,
not a violation — the §Fail-closed contract, and the same verdict `gate_command`
already gives, since "cannot verify" and "verified fresh" must not share an exit
code.

**Executability has one spelling in the crate, `proc::is_executable`, and this
module is not a second one.** The module carried a private copy taking
`std::os::unix::fs::PermissionsExt` with no `cfg`, duplicating a predicate
`proc` already twins across `#[cfg(unix)]` / `#[cfg(not(unix))]`; the copy is
deleted and the call site reaches the shared one. Enforcement-first prefers
de-duplicating the copy to gating it, and the copy was the crate's **only**
un-gated unix-API use — the whole reason the crate would not compile for
`x86_64-pc-windows-msvc` at all, where `cargo build --target` failed on the
copy and `build-native.sh` propagated cargo's own non-zero status.
**One behavioural difference rides the de-duplication, in the right direction:**
the deleted copy returned true for any path with an execute bit, a directory
included, where the shared predicate additionally requires `is_file()`. This gate
uses the predicate to decide whether the path `GATE_SDK_NATIVE_BIN` names is a
runnable binary, and a directory there is not one, so the shared predicate is
strictly more correct at this call site. On non-unix the twin is `is_file()`
alone, the honest answer where the filesystem carries no execute bit —
executability on Windows is an *extension* question, and that question belongs to
`gate_exe_suffix` (§lib/gate.sh), not here.

**Why this is deliberately *not* assertion B's split-halves shape, which a reader
one section away will otherwise think is owed here.** Assertion B was corrected to
run its roster half whenever the binary is merely *readable*, descriptor count
irrelevant, because a single `descriptors > 0` guard made the whole assertion go
dark in precisely the unported state. That correction does not transfer, on two
grounds this section states rather than leaves to be re-derived. A stranded
subcommand is a defect whatever the descriptor count; a stale binary **nothing
dispatches to** is not a defect at all — it is an unbuilt artifact. And comparing
whenever a binary happens to be readable would make the verdict depend on whether
a given clone ever ran `cargo build` — green on a machine that never built, red on
one that did — which is the build-time coupling the revert removed, re-imported
into the auditor through the back door (§check-gate-substrate-parity).

**Why a new gate rather than a seventh assertion on
`check-gate-substrate-parity`.** Not for its trigger: `native/*` is a bash pattern
in which `*` spans `/` (§run-gates), so the parity gate already re-fires on every
crate edit and folding this in would buy no trigger it lacks. The reason is the
dependency. Parity **stays a shell gate that never consumes the substrate it
audits** — its own section says so, because assertion B checking a roster through
the very binary whose roster is in question is circular. Freshness must invoke the
binary to get its verdict, by construction. Acquiring that dependency is precisely
what parity's rule forbids, so the two cannot be one gate. The subjects differ to
match: parity audits what a port *declares*, this audits whether a build is
*current*.

Knobs: `GATE_SDK_NATIVE_BIN` and `GATE_SDK_NATIVE_CRATE`, both existing
(§Layout and configuration) — the gate introduces none, and a knob selecting the
registry would be a second spelling of `GATE_SDK_GATES_DIR`. The registry is a new
**input** rather than a new knob: it is `<gates-dir>/gates.list`, the path the
positional argument already determines, so the fixture pair stays hermetic.
`# graph:` manifest: `tier=precommit`, coupling the crate root (the source tree is
the subject), `kit:checks/*.gate` (the descriptor set) and `scripts/gates.list`
(the registry — a registration edit alone can flip the verdict, so it is a trigger
like any other subject).

Its coverage is split across three oracles, and naming where each is proved is
what keeps the split from reading as a hole. The `good/`+`bad/` pair covers
**descriptors present and registered, binary readable** — the comparison reached
and agreeing, and the mismatch red, with the descriptor set derived across both a
gates dir and a kit resolve dir. It is hermetic through the second positional
argument, which supplies the tree-side stamp from a file instead of computing it,
the shape §check-exec-bit uses for the same reason: a fixture tree cannot carry a
git index, and a fixture crate would break in a vendored tree whose files are not
yet committed. The bespoke `gate-tests/check-gate-binary-fresh.test.sh` covers the
predicate's boundary, which a pair cannot hold: **zero descriptors** — the
standing state of an unported tree, held in a fixture rather than assigned to a
live tree for the reason §check-gate-substrate-parity's own split states; **descriptors
present, none dispatching** — every vendored tree since the first cohort shipped, and
the configuration the corrected predicate exists for; the **near miss**, a
registered member resolving to a descriptor with no binary, which must still exit
2; and an **absent registry**. And the
git computation itself is held by an **executed** assertion rather than a static
one: a crate unit test compares the baked constant against
`gate_native_source_stamp`'s output over the real crate, failing on any drift
between the two substrates' invocations. That test is the precedent the
`check-knob-default-coupling` disposition sets (§Meta-gate conservation for the
binary substrate). **Its standing changed with this cohort and the change is
recorded rather than left to a reader to notice.** It was load-bearing because a
hermetic fixture plus a zero-descriptor tree meant *nothing in the gate battery
ever executed the tree-side computation*; this repo now registers both first-cohort
members, so the battery reaches that computation on every run and the test is no
longer the only executor. It is kept, on the narrower ground that it fails at
`cargo test` time — inside the crate's own suite, before any binary is placed —
where the battery's verdict arrives only once a build exists to compare against.

### check-crate-arms

The crate's **lint and test arms** run at commit time, as a registered member of
the battery:

```bash
cargo clippy --release --manifest-path "$CRATE/Cargo.toml" --target-dir "$TARGET_DIR" --all-targets -- -D warnings
cargo test --release --manifest-path "$CRATE/Cargo.toml" --target-dir "$TARGET_DIR"
```

`$CRATE` is `gate_native_crate` (§Layout and configuration) and `$TARGET_DIR` is
`GATE_SDK_CARGO_TARGET_DIR`. Both arms run even when the first fails, so one
commit-time report carries what a CI run would have said in two.

**The gap it closes was attested, not hypothetical.** Nothing in `gates.list` ran
either arm and `bin/build-native.sh` only builds, so a contributor who ran the
full battery **plus** `build-native.sh` had satisfied every documented commit-time
obligation and could still push a red CI — and did: both arms were already red at
a commit accepted on a green battery, one of them the unit test holding
§check-reads-couples' machine-side read-set assertion, which therefore held
nothing for any member while the conservation table still claimed it. The lesson
the shape encodes is that a *nearly* identical command is the same hole one flag
narrower, so **there is now one spelling of the arms and it is this gate**. The CI
workflow's own lint-and-test step is deleted rather than kept in step with it:
two copies of a command held equal by nobody is the defect, and enforcement-first
ranks removing the duplication above gating it. The workflow's preceding
build-the-binary step stays — §check-gate-binary-fresh still needs the artifact,
and that step duplicates nothing.

**The predicate is the crate's presence, not cargo's**, and that is what keeps the
gate simple:

- **No `$CRATE/Cargo.toml`** — no corpus, so **clean at exit 0**, the
  parenthetical naming the absent crate so the green is legible. §Consumer payload
  keeps the crate outside every kit root, so what a consumer tree lacks is the
  subject, not the toolchain. This is §check-gate-binary-fresh's zero-dispatch
  branch applied to a corpus: a gate with nothing to check reports clean rather
  than declaring a skip.
- **Crate present, `cargo` absent** — a contributor holding the crate without the
  toolchain. **Exit 2** naming the floor, the same shape `bin/build-native.sh`
  already uses (§Fail-closed contract). No adopter reaches it, and a session
  committing to a tree that carries the crate needs cargo before this gate runs.

Reading a runtime `command -v` skip into §The port-candidate criteria's criterion
5 is a misreading worth naming, because the queue entry that filed this gate made
it: criterion 5 omits a member from the *consumer's* registry at vendor time, a
decision `init` makes once before any gate runs, and no gate in this tree branches
on a missing program at run time — both existing cases fail closed.

Its `# install:` disposition is `never`, for §check-gate-binary-fresh's reason
exactly: the subject cannot exist in a vendored tree, so the consumer smoke's
accounting derives the exemption every run rather than accepting a written one.

**Tier is `precommit`**, by §The `# graph:` manifest's discriminator: a clippy
finding or a failing unit test is introduced and repaired inside the single commit
that perturbs it. The honest counter-pressure is wall clock, since `--all-targets`
compiles the test targets and clippy keeps a cache separate from the
`cargo build --release` `bin/build-native.sh` warms — and the claim this paragraph
first made, that the added time sat inside the battery's dominant member, **stopped
holding**: measured 2026-08-23 the gate was 27.2s of a 52.4s battery, the
dominant member by a factor of four. The ruled answer is the source-stamp cache
the next paragraph states, not `align-only` and not a narrowed arm set: a cached
green is the same two arms' verdict over the same source and toolchain, so the
commit-time claim keeps its full content at the cost of one hash.

**The fixture pair and the build products it must not leave behind.** The pair is
two minimal crates, `good/` clean and `bad/` carrying one clippy finding and one
failing test, each `args`-free and pointed at its own crate through its case-dir
`gate-sdk-config.sh`. They build with no network because they vendor nothing —
the property the real crate's own suite asserts and these inherit by
construction. Cargo writes a `target/` beside whatever manifest it is given, which
would put build products under a fixture directory, so each case redirects
`GATE_SDK_CARGO_TARGET_DIR` out of the tree; the `Cargo.lock` cargo writes beside
the manifest is gitignored on the same terms as the real crate's.

**The gate ported — ruled 2026-08-23, reversing the permanent-shell verdict this
paragraph used to carry.** The two justifications it stood on are recorded with
their refutation at §The port-candidate criteria, criterion 7: the rule *is* an
invocation of `cargo`, which a compiled wrapper spawns exactly as the shell does;
and "a gate running `cargo test` over the crate cannot live inside the crate it
tests" conflated the installed artifact with the source cargo compiles afresh.
It is `install: never`, so no adopter receives it under either substrate.
**Its commit-time cost is
bounded by a source-stamp cache** rather than by narrowing its arms: the two
arms re-run only when the crate's source stamp (§lib/gate.sh,
`gate_native_source_stamp`) or the toolchain differs from the last green run
recorded under `.tmp/`, and a crate with untracked files never hits the cache.
CI and a fresh clone carry no record, so they run both arms in full — which is
what keeps the battery's claim that a passing commit cannot coexist with a
failing CI true at the site it is made.

**It is `.gate`-dispatched since `shell-gate-tail-port`**, declared at
`gate-sdk/checks/check-crate-arms.gate` with its rule in
`native/src/gates/crate_arms.rs` — the port that emptied `gate-sdk/checks/` of
its last `.sh`. It consumes the wrapper contract rather than extending it
(§Fail-closed contract): `proc::on_path` for the presence probe and
`proc::run_merged` for each arm's `2>&1` capture, because for these two the
**failing** run is the one whose report has to print. The one face it adds is
`Merged::reported_code()`, for a wrapper that *prints* an exit code rather than
branching on it.

**It declares three programs where criterion 7's report counts two.** `cargo` and
`rustc` are the off-floor pair; `git` is the third, observed through
`gate_native_source_stamp`'s compiled counterpart and on `GATE_SDK_PROGRAM_FLOOR`,
so it earns no criterion-7 residual and the report never had cause to name it.
All three are declared because unit test A asserts *observed ⊆ declared* and an
undeclared program is exactly what that direction is for.

**`rustc` is read for the cache key and nothing else, so its absence is not a
refusal** — stated because the natural reading of "declares two programs" is that
each gets a refusal arm, and building one here would be a behaviour change wearing
parity's clothes. The key is composed from each program's version output with its
stderr discarded and its emptiness untested, so an absent `rustc` contributes an
empty field, which is a cache **miss** against any key
written while it was present: the arms re-run and cargo reports whatever a
toolchain missing its compiler reports. Only
`cargo` has a refusal arm, and it fires **after** the crate-presence branch — a
tree with no crate is clean whether or not cargo is installed, which is the
per-member probe point §Fail-closed contract says to read off the member's own
ordering rather than inherit from a sibling wrapper.

**Both arms still run when the first fails**, which the port had to be written for
rather than inherited: the natural compiled shape short-circuits on the first
non-zero status and would report a clippy finding while never running the tests,
silently halving the report the gate exists to give in one piece. The second
spawn is unconditional on the first's verdict, and that is the whole of the rule.

**The source stamp is one implementation, now shared.** §check-gate-binary-fresh
holds its tree-side stamp to being one algorithm rather than two, which is what
makes its verdict mean anything; this member caches on the same value, so the
runtime helper lives in `native/src/fresh.rs` and both members call it rather than
each carrying a copy. The shell spelling is unaffected and still has its own live
reader — the cross-substrate comparison in `main.rs` that holds the two
substrates' derivations equal.

**Criterion 4 binds, and the live-tree arm was not demoted.** The pre-port rule was
restored under a non-resolving name inside the resolve dir, so both forms read the
**post**-descriptor corpus, and the arm carries **no** bound of the kind
§check-shellcheck records: this member's corpus is the crate cargo is handed by
manifest path, so the restored `.sh` sits outside the corpus it probes.
Fifteen comparisons covering both sides of the cache, the untracked and
no-tracked-source paths that skip it, and the constructed PATH-scrubbed scenario
run three ways — `cargo` absent, `rustc` absent, and both absent — each against a
crate-present and a crate-absent tree, so the probe-point ordering is proved on the
arm where the two branches disagree. Fourteen are byte-identical including exit
codes. **The fifteenth is recorded rather than rounded off**: on the `bad/` case the
two runs differ in one number, the process id inside the panic line *cargo* prints
for the failing test. That is the child's own non-determinism reaching a report this
gate forwards verbatim, and it is a property of the report rather than of either
substrate — run twice, **the shell form disagrees with itself in the same place and
one more**, cargo's `Finished … in 0.02s` timing line. Naming it keeps the parity
claim's meaning: what is proved identical is everything this member decides, and a
relayed report is not one of those things. It is also why the pair's `bad/`
expectations match substrings rather than pinning the report.

### check-install-disposition

Three assertions over every kit root in `gate_kit_roots`, holding §The install
disposition.

- **(A) Declared** — every `checks/` member, `check-*.sh` and `check-*.gate`
  alike, carries **exactly one** `# install:` line and its value is in the closed
  vocabulary. A gate that ships without one is red, and this is the assertion
  that closes the live exposure: a newly added zero-config gate cannot reach an
  adopter undeclared. Two lines are a finding for the same reason one is
  required — a declaration surface's failure mode is disagreeing with itself.
- **(B) Smoke superset** — every `zero-config` member of a kit appears in that
  kit's `smoke/install.sh` roster. The direction is what the two-tree ground
  gives: the smoke's tree is a superset of the tree `init` makes, so a gate the
  installer registers must be registrable there too. **The converse is not
  asserted** — a kit's smoke legitimately registers more than the installer does,
  and asserting equality would force one of the two trees to lie about itself.
- **(C) No second copy** — the installer's `lib/common/recipe.sh` carries no
  literal gate name, so the de-literalization holds going forward rather than
  only at the commit that landed it. A `§`-prefixed occurrence is a spec-section
  citation and is stripped before the match: a section reference registers
  nothing, and registering is the only thing this assertion is about. The file is
  absent in a vendored consumer, which has no installer — that absence is a skip,
  reported on the clean line, never a finding.

Fail-closed on a non-repo cwd with no root argument, an unreadable gate header,
an empty kit roster, or kit roots that enumerate no gate at all. Configuration
reuses `GATE_SDK_KIT_DIRS`; the gate adds no knob of its own. `precommit` tier.

**Its implementation is a compiled subcommand** since `shell-gate-tail-port` —
declaration path `check-install-disposition.gate`, rule out of the gate binary,
the shell original deleted. All four fail-closed arms above survive the port; what
the port narrows is what the positional root is *for*. The kit roots arrive over
the config bridge already spelled against the invoking directory (§lib/gate.sh),
so the compiled form anchors them there, and the root argument — with the git
toplevel it falls back to, which is this member's one declared external program —
anchors the installer recipe alone. Every invocation in this tree runs at the
directory the roots were spelled against, so the two anchors coincide; a caller
passing a root that is not its cwd reads the recipe there and the kit roots where
it stands.

**Criterion 4 binds, and the fixture pair is what discharges it** (§The
port-candidate criteria, criterion 4): every registry member's declaration path
lies inside the corpus this gate scans as content — its own included, and in both
spellings. The pair carries the four arms of that derivation: a `.sh`-declared
member, a `.gate`-declared one, a **mixed** kit holding both, and assertion C's
absent-recipe skip. The skip is carried by `good/`, which ships no installer at
all — the vendored-consumer shape — while `bad/` carries the literal-roster
finding beside a `§`-prefixed citation that registers nothing. The `.gate` arm is
a **finding** rather than a silent count on purpose: a descriptor the sweep counts
but never reports would leave the second glob asserted by nothing, which is the
`.gate`-arm-exercised-by-no-case defect the README-roster widening already named.
What a pair cannot assert is an absence, so the `§`-stripping and the
`# install:`-with-no-value reading are held by crate unit tests beside the rule.

**The live-tree arm did not have to be demoted here, and the technique is why.**
Criterion 4 records that arm as smoke for a gate-source member, because
§check-gate-substrate-parity assertion A forbids a descriptor and a script
coexisting in one resolve dir, so the comparison runs on a pre-descriptor rev. The
restriction is on a **resolving** name: restoring the pre-port rule under a name
no registry member resolves to puts both implementations over the *post*-descriptor
corpus, which is what ran here — both forms read the same live corpus, this
member's own descriptor included, and agreed byte for byte on stdout, stderr and
exit code over the live tree, both fixture cases, and both fail-closed arms no
committed fixture can reach.

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

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-test-hermetic.gate`, rule out of the gate
binary, proved parity-identical before the shell gate was deleted
(§The kit-roots `gate_kit_roots` cohort). Its two `# assertion` markers moved
with the rule to the implementation module, which is where §check-gate-assertions
now looks for a `.gate`-declared member's.

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

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-assertion-strength.gate`, rule out of the gate
binary, proved parity-identical before the shell gate was deleted
(§The kit-roots `gate_kit_roots` cohort). The three bounded matchers and the
`# exit:` line grammar are hand-written kit literals, not an interpreted
consumer pattern, so this member owes none of the ERE-engine work its held
siblings do.

### check-gate-exemption-tasks

Invariant: **every temporary-disposition annotation in scope names a live task** —
on a gate declaration, and on any tracked script for the held-port field. Two
annotations are in scope. Every element of an
`# exception-list:`-tagged array in a
`check-*.sh` gate carries exactly one of two disposition annotations —
`# until: <slug>` (temporary; must resolve to a live task in the queue file's
New Features / Technical Debt / Deferred sections — *live* meaning the slug on a
**bullet lead line** within that span, one per entry, never every bold token in
it) or `# permanent: <reason>`
(structural out-of-scope). An element with neither, a `# until:` slug that is
Done-only or missing, or elements sharing the array's opening `=(` line are
violations. And every `# port-until: <slug>` **header field** on a declaration
(§The `# graph:` manifest) resolves through the same live-slug map; a slug that is
Done-only or missing is a violation. Inline per-site
directives (`# fail-closed-exempt:`, `# no-fixture:`) stay out — they are
local and self-evident via their adjacent comment.

**The held-port field's corpus is the tracked shell tree beside the declaration
set, and it widened with the field itself** (§The `# graph:` manifest). Without
this the field's widening would ship its own worst failure direction: an
**undeclared** hold is counted owed, the status quo a reader's own audit catches,
while a **stale** declaration whose blocker landed under-counts the owed set and
hides real work — the direction no shape assertion covers, which is why a slug is
held to a live queue entry rather than to a shape. It is the same assertion over a
wider walk and not a new one; what changed is which files it collects slugs from.

**A union, never a replacement, and that is not a nicety.** A `.gate` descriptor
is no `*.sh`, so a corpus that replaced the declaration set with the tracked shell
tree would silently drop every descriptor-borne field. The walk is therefore the
declaration set plus the tree corpus §port-blockers derives, de-duplicated against
**both** halves of the declaration walk — the in-scope set and the out-of-scope
set — which also makes the widening **monotone**: it can add findings and never
remove one, so no existing verdict can flip by inspection failure. De-duplicating
against the in-scope half alone double-counts a kit-shipped declaration on the
skipped tally, which is what the fixture pair's out-of-scope count caught.

**Where there is no tracked set the tree half is empty and the declaration half
still asserts**, which is the opposite disposition from §port-blockers' `--tree`
arm and is deliberate. That arm's *whole* subject is the tracked tree, so a
non-repository leaves it nothing to answer and it refuses; here the tree corpus is
an **addition** to a corpus the gate can still read, so degrading to none returns
exactly the pre-widening assertion — monotone again, and the same direction
`gate_authoring_tree` already degrades in when git cannot answer it. An unresolved
prune knob is a different thing and still fails closed: that is misconfiguration
rather than an absent corpus. Caught by a bespoke test that stands its sandbox up
outside any repository, which is the one arm no fixture pair reaches.

**The scope rule is the declaration corpus's own predicate lifted from a directory
to a file**, so an adopter is never held to a kit author's slug: a tracked script is
in scope iff this tree authored the kits it carries, or the script sits under the
consumer's own gates directory. That is the same `gate_authoring_tree` test the
paragraphs below already turn on, and it is stated as a lift rather than a new rule
because it decides nothing the declaration arm had not already decided. **Its
bound, stated rather than banked:** in a *vendoring* tree a consumer's own scripts
outside its gates directory are not asserted. Under-assertion is the safe direction
here, it is exactly what the declaration arm already does, and closing it means the
per-kit authorship marker the limit paragraph below already prices as a different
unit.

**A disposition is read from the file's header block alone over the tree corpus,
and the declaration corpus keeps its whole-file scan.** This corpus contains
scripts that *write* shell — smoke scripts, installers, template authors — and a
line-anywhere scan cannot tell a declaration from a heredoc literal. The
restriction is the field's own name rather than a new rule, `# port-until:` being a
**header** field; confining the tree read to the leading run of shebang, comment
and blank lines removes the false-positive class by construction. Leaving the
declaration corpus alone is what keeps the widening monotone: narrowing a scan that
already shipped would retire findings, which a widening may not do. **Found by
running the widened gate over this tree rather than by reasoning** — its first live
run reported a hold against a heredoc literal in `gate-sdk/smoke/` — and recorded
because the asymmetry between the two corpora is deliberate and reads like an
oversight.

**The widening was taken over a closer shape-fit rival, and on cost.** A
top-of-file header field is a different syntactic subject from a per-element
trailing comment, and on shape alone §check-gate-substrate-parity's assertion G —
which already reads header fields on this exact declaration set — is the closer
home. It loses because the two annotations make **one claim about one queue**:
this gate holds the live-section span, the bullet-lead-line predicate and the
queue-file coupling **today**, so the invariant generalizes with **no** new holder
of a predicate the cost paragraph below prices at five. The rival would have made
that a sixth, and would have added a queue-file coupling to a gate that
deliberately has none. **The corpus half of that argument has since expired and the
ruling survives without it**: the two corpora coincided when the arm landed here,
and the field's widening to any tracked script parted them, so this gate now walks
a corpus assertion G does not. The five-versus-six accounting is untouched, and it
was always the load-bearing half — a second holder of one claim about one queue is
the cost, whatever the corpora do.
The spelling collision with `# until:` is the precedent being cited rather than an
accident to rename: the two differ in subject and in prefix and are read by one
liveness predicate, and a reader who greps `until:` finds both, which is correct.

**Both arms are scoped to the tree that authored the declaration, and the rule
is a loosening this repo's own iteration paid for.** A temporary disposition
names a task in *a* queue, and the only party that can make that task land is the
one that owns the queue. A vendored kit's declaration is the kit author's, shipped
read-only into a tree whose queue never carried the slug and never will, so
asserting it there demands of an adopter something only the kit author can
satisfy — and reds every freshly initialised consumer's battery on gate-sdk's own
files. **A declaration is in scope iff it is under the consumer's own gates
directory, or this tree is the one that authored the kits it carries**; the
authoring test is `gate_authoring_tree` (§lib/gate.sh), the same predicate
§check-gate-substrate-parity's assertion B already scopes by, shared rather than
spelled twice. In the authoring tree nothing is out of scope and the assertion is
exactly what it was.

**The rule is the class, not the instance.** Both annotations take it — the
`# exception-list:` element's `# until:` as well as the declaration's own
`# port-until:` — because the exposure is identical by construction and the array
arm is un-instantiated across the shipped kits only by accident. Scoping the arm
that happened to fire would leave a scheduled recurrence for whoever next ships an
exemption array from a kit.

**The skipped set is counted on the clean line**, which is what keeps the
loosening from becoming a silent stop: a scope rule that quietly widened is
otherwise indistinguishable from a corpus with nothing to assert. The count is
non-zero exactly in a tree that vendored a kit shipping such an annotation, and
zero in the tree that authors them.

**Its honest limit, stated rather than banked: the predicate is tree-shaped where
the question is kit-shaped.** A consumer that authors its *own* kit beside
vendored ones reads as non-authoring, so its own kit's dispositions stop being
asserted — its consumer-gates-dir declarations still are. That is the same limit
§check-gate-substrate-parity's assertion B already carries for the same predicate,
and narrowing it means a per-kit authorship marker, which is a different unit.

**The header-field arm enters the walk independently of the `# exception-list:`
marker.** The array arm opens on that marker, so a declaration carrying only a
header field would be skipped by it — the arm is a second entry into the walk
rather than a widened regex, and since the field widened it is a second entry into
a **wider** walk: the `gate_check_dirs` declaration set plus the tracked shell
tree. A **bare** field with no slug
is assertion G's shape clause and is passed over here: there is no slug to
resolve, and reporting it in both places would give one defect two reds whose
wording disagrees about what is wrong.

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

**The honest cost, stated with its size: five independent holders now carry
that predicate and no gate enforces their agreement** — `queue_live_slugs`
(queue-kit/lib/queue.sh), `queue_slugs`
(native/src/spec.rs — canon-kit's holder, which moved substrate with the seventh
cohort and is deliberately not pointed at the crate's own queue module, since one
shared function would end the arrangement this count prices), the crate's own
queue module, the section-pool builder inside
drift-kit's `kpi-queue-net-delta`, and this gate. **It was eight, and porting is
what shrank it** — the roadmap adapter left the shell library for the crate's own
queue module when its two consumers ported together, so a holder became a second
call site of one already counted; the inline scans in `check-task-names` and
`check-queue-entry-budget`, and both of the `queue-index` arm's walks, now call
one shared crate function, and two call sites of one function are one holder by
this section's own criterion. That is the shared-derivation question below
answering itself for three members without anyone deciding it. Only the public
library functions are named; the rest are cited by the surface that owns them,
because naming another kit's private helper here would couple this
count to an identifier no gate holds. They are cited at all because a grep for the *function* names
finds two of them and the rest are inline scans no naming convention surfaces —
which is how the count was twice under-stated before it was surveyed. The risk
and the cost do not sit in the same place: a **set builder** with a wrong
predicate fails silently, in wrong membership, which is this gate's own defect
class, while a **per-bullet extractor** fails loudly and locally — a missing
index row, an extent measured wrong. So a format change costs all six edits
and endangers only the few that build sets. Accepted on the same ground as the
section-set residue above: a cross-kit code dependency would cost more than the
divergence risk. **The `# port-until:` widening added no holder to this count**,
which is the reason it landed here rather than in
§check-gate-substrate-parity — it reuses the `IS_LIVE` map this gate already
builds, at a second entry into the walk it already makes. Whether the remaining hand-coupled parsers earn a shared derivation, a
conformance test, or a gate is a real question and a **different unit**; this
section neither answers it nor forecloses it.

**Ported to the binary substrate at §The sixth budget batch.** Three things the
port had to get right are recorded here rather than left to the module. Its
`[queue-file [dir…]]` positional pair **ports unchanged**, and the dirs positional
is independent of the scope anchor: it says *what* to read, while
`gate_authoring_tree` and the gates directory say *whether* what was read is in
scope, so overriding one never moves the other. The queue-format literals above
stay literals in the compiled form too — the crate holds a *different* member's
section reader that takes knobs, and copying that shape here would silently
acquire the knob this section refuses. And the port adds a **third in-crate holder
of the lead-line predicate while leaving the count at five**: the shell original
left with it, and whether the hand-coupled parsers earn a shared derivation is
still the different unit named above.

**Criterion 4 binds in every configuration, which is a new row for the register.**
In an authoring tree this member's own declaration is in the scanned set; in a
vendored consumer it is in the *out-of-scope* set, which is still read to build
the skip count. There is no configuration that clears it, so the criterion is paid
by the pair rather than answered by a corpus that misses the member — and the
pair's `.gate` half was the hole the port closed: both cases now ship a descriptor
on each side of the scope rule, because a glob arm no case reaches is an arm the
live tree cannot exercise either.

Its declaration corpus is resolved by **one-level pathname expansion** over each
listed directory, and its tree corpus by **enumeration of tracked files**, so the
compiled member still declares an empty read-root set and `--reads` still reports
nothing to cover (§check-reads-couples). Neither half is a recursive walk, which is
what the read-root set is a set of: a `git ls-files` spawn asks the index rather
than descending a directory, so the recorder observes no root — **verified by
running that gate rather than reasoned**, since the opposite prediction is the
natural one. What the tree corpus does add is one bridged knob, the prune-dir set
`--tree`'s corpus rule reads, which the member declares like any other: a knob read
without being declared is the bridge's undeclared-knob refusal on every invocation.

Clean-line contract: the line reports the exemption-array count, the
`# port-until:` header-field count, the **out-of-scope kit-shipped declaration
count** and the
derived live-slug count. All four are §run-gates' vacuous-pass tripwire applied to
this gate's sets, and they read in opposite directions — an empty array or
header-field set means that arm ranged over nothing, while an absurdly *large*
slug set is the
fail-open above, silent by construction because every `# until:` then resolves.
A number on the line is what makes the last readable without an audit, and its
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
`GRAPH_LAYERS` (the projection's subgraph grouping; absent renders one layer),
and the layer lookup as **data rather than a hook**: `GRAPH_LAYER_RULES`, an
ordered array of `<path-prefix>:<layer-id>` elements split at the **last** `:`
(a layer id is `[A-Za-z0-9_]+`, so a prefix may itself carry one) with **first
match wins**, and `GRAPH_LAYER_DEFAULT`, the layer a surface no rule matches
takes (kit default `surfaces`). The file is sourced by `lib/gate.sh`, not by the
member: the member is compiled and receives the resolved *values*, so
`GATE_SDK_GRAPH_VOCAB` is a path the crate never sees and never declares, and
all six globals are defined before the source so an absent file resolves to
empty arrays that disable their checks exactly as before.

The rule is a **prefix test, not a glob, and that is a deliberate narrowing.**
The retired `graph_surface_layer()` hook accepted an arbitrary shell pattern over
the whole path, unanchored to path segments; the crate carries a component-wise
matcher and a slash-spanning one side by
side and nothing says which a port should reach for
(`couples-glob-semantics-unowned`). A prefix test has no glob semantics to own,
expresses every rule the live consumer's hook expressed, and closes this
surface's exposure rather than adding a fourth reader of an unowned question. The `--amend-only [dir]` mode runs only
(G) over a given directory, letting the fixture pair exercise it hermetically.
**Assertion B's coverage predicate, stated because it is the `couples=` field's
third reader and it invokes no glob matcher at all.** A couple is covered when
any trigger token satisfies one of four branches: a `*` trigger covers
everything; an exact string match covers; a **literal** couple — one carrying no
`*` or `?` — covered by the trigger read as a bash pattern covers; and a
`*.<ext>` trigger covers a couple ending in that suffix. Neither of the crate's
two matchers *is* this predicate — the component-wise one requires equal segment
counts and the slash-spanning one is branch three alone — and substituting
either flips verdicts on the live registry, so the predicate is carried whole.
Criterion 6's globstar commitment (§The port-candidate criteria) governs a Rust
glob matcher over a bridged knob and does not reach a predicate that matches no
glob; it is stated here because it is the first ruling a porting session finds
and it is the wrong one for this reader. This closes the **port's** exposure to
`couples-glob-semantics-unowned` and that entry's undocumented-third-semantics
half; whether `couples=` has one semantics with stated exceptions or a per-reader
meaning declared per reader is untouched and stays that entry's deliverable.

Coverage ruling: a full `couples ⊇ find-globs` parity check over arbitrary
shell is undecidable — neither cheap nor low-FP — so check-graph does not carry
it, and the couples⊆trigger guarantee already ensures editing a *coupled*
surface fires the gate. The
statically resolvable slice of that parity is carried by its sibling
`check-reads-couples` (§check-reads-couples); the undecidable remainder stays
the author's duty under §The `# graph:` manifest.

Port sizing, **corrected at the port**: the 929-line figure recorded 2026-08-17
counted the generator, and `bin/gen-pre-commit.sh` (297) does **not** port
(§gen-pre-commit). The ported surface is `check-graph.sh`'s 632 lines; the
generator's 297 stay shell with the cause stated there, and assertion D keeps
spawning it for `--emit` and `--emit-commit-msg`. The spawn-invisibility rule the
old figure illustrated still holds for a generator that *is* in a port's scope —
what was wrong was the assumption that this one was.

**The runtime paragraph is corrected against measurement, and the correction
removes an argument rather than adjusting it.** This member is among the
battery's slowest, and the reason is the two spawns that do not port. Re-measured
against this tree after the config bridge began batching by owning kit
(§lib/gate.sh), median of three: the member runs 4546 ms, of which
`gen-pre-commit.sh --emit` is 4153 ms and `--emit-commit-msg` 219 ms — so the two
spawns are still very nearly the whole of it, and the `--emit` figure is still
the config bridge resolving argv for every registered member, now once per owning
kit rather than once per declared knob. The batching is where the fall came from:
the same three numbers read 7629 / 5651 / 210 before it. **The shape of the
finding is unchanged and that is the point** — the spawns dominate whatever their
price, so a cheaper bridge moves the figure and not the argument. What the port
banks is the graph emission and the per-member manifest read, in process. The
earlier claim that *the port makes those calls in-process* was false as written:
it never could, under the generator's own cause for staying shell. The operator
ruling of 2026-08-09 is the argument for this member's port and it needs no
other; a runtime dividend was never one. Per-gate timings stay owned by the
consumer's timing baseline and the close-stage runtime review that reads it,
never by this line.

Theme seam: the emitted HTML artifact bypasses the consumer's
site generator, so it renders foreign beside the rest of a docs host unless the
host theme is inlined — and the theme is consumer-specific, so the emitter must
not hardcode it. `GATE_SDK_GRAPH_THEME_DIR` (default `<gates-dir>/graph-theme/`)
names a directory of at most three optional part files, each inlined **byte
verbatim** at one injection point:

| part file | injection point | absent |
|---|---|---|
| `theme.css` | the `<style>` element's body, replacing the kit default stylesheet | kit default stylesheet |
| `header.html` | directly after `<body>`, above the kit header | nothing emitted |
| `footer.html` | directly before `</body>` | nothing emitted |

The roster is closed at three: a part file with no injection point is not
defined. An absent directory or an absent part falls back exactly as an undefined
override function did under the retired seam, so a themeless consumer's output
stays byte-identical. The kit neither adds nor strips a trailing newline — a part
file's own bytes are what appear.

**Why a directory of files rather than bridged values.** The config bridge
(§lib/gate.sh) refuses any element containing a newline, exit 2, because the
newline would break the line-per-element argv protocol; a stylesheet and two HTML
fragments are newline-bearing by construction, so theme *content* cannot ride the
bridge at all. Only the **path** crosses, relative, and the binary reads the files
itself. That is the general rule §lib/gate.sh states: values cross the bridge,
documents cross as a path.

**This replaced a sourced-function seam and the replacement is breaking, stated
rather than presented as a clean port.** `GATE_SDK_GRAPH_THEME` and the three
`graph_theme_css`/`_header`/`_footer` override functions are **retired**: a
consumer carrying a `graph-theme.sh` had that file read at every emission, and
after this change it is not. What the operator ruling of 2026-08-20 bound the
replacement to is that the doctrine holds through the cut: CLAUDE.md §The
provenance seam names this pair as its worked example, and the theme stays the
consumer's — only its **form** moves from executable to declarative.

**The retired seam fails loudly rather than silently.** The gate refuses, exit 2,
naming the migration, when `GATE_SDK_GRAPH_THEME` is set in its environment or a
file exists at `<gates-dir>/graph-theme.sh`. The tripwire is permanent kit weight
and it is worth it: a themed consumer that silently lost its theme produces a
regenerated artifact the byte-compare cannot distinguish from a legitimate theme
edit, so the failure would be invisible in a green battery — the exact shape
§Fail-closed contract exists to refuse.

Determinism: the freshness assertion's in-memory emission and the emit arm a
consumer redirects into the artifact read the same part files, so the
byte-compare holds; the artifact stays generated-only, a styling change landing
in a part file (or the emitter), never a hand-edit. The node set is byte-sorted,
which is deterministic across locales — a narrowing the port took on purpose,
since a locale-dependent ordering makes the artifact machine-dependent and the
byte-compare reads that as drift.
Self-containment is unchanged: injected content is inline, and a theme emitting
a relative asset href must resolve under the artifact dir or the asset-href
assertion is red — the existing gate already polices the link-the-site-stylesheet
shortcut into inlining. Its complement is the **external-ref assertion**, over
the same in-memory emission: every absolute (`://`-carrying) `href`/`src`
attribute value and every ESM `import` specifier must prefix-match the allowed
set, or the gate reds naming the URL. The set is seeded with the emitter's own
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
with a dark-rendered graph on the same page. This repo's `scripts/graph-theme/`
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

**Criterion 4 binds on this member, under every configuration.** The gate
resolves every `gates.list` member's declaration path and reads its bytes for the
`# graph:` line, so a registry member's declaration path lies inside the corpus
the gate scans as content — criterion 4's predicate exactly — and there is no
consumer config in which the registry is not that corpus. It joins
`check-gate-exemption-tasks` as a member whose verdict flips on nothing. The
discharge is the fixture corpus, and its condition is that the corpus carry
**every arm of the derivation being ported**. Measured at spec on 2026-08-21 it
carried **one arm of nine** — `good/args` and `bad/args` were both
`--amend-only`, so only assertion G was fixtured and every other assertion rested
on a live tree that is green because it is clean. The widening therefore landed
**before** the port, at nine of nine, and it splits by assertion shape:

- The `good/`+`bad/` pair carries assertion **G and only G**. A case runs one
  argv, so it selects one mode, and the whole-tree default reaches D and E —
  which a `good/` case cannot satisfy, since
  `gen-pre-commit.sh` cds to `git rev-parse --show-toplevel` before resolving the
  gates dir, and satisfying E would take a committed `CHECK-GRAPH.html` inside
  `gate-tests/`. Both are the second-copy-of-a-generated-file this corpus
  refuses. **`--amend-only` is not the *only* mode that escapes D and E, and this
  clause once said it was.** `--refs-only` reaches H alone and `--cap-only`
  reaches I alone, both returning before the whole-tree path — the same two modes
  named as hermetic drivers three bullets down. What makes `--amend-only` the
  pair's mode is that its corpus is fixturable in-place, where H and I read an
  emitted artifact their own drivers construct. A fourth mode selecting the
  registry assertions is refused too: it
  would add consumer-visible surface to buy a corpus a driver already has.
- Assertions **A through F** take a constructed mini-consumer in
  `check-graph-tree.test.sh` — a `mktemp`'d git repo carrying a registry, five
  member declarations in **both spellings**, and a vocabulary file. Its `.gate`
  member is `tier=align-only`, which keeps it out of both generated hooks and so
  out of `gate_command`'s reach, letting the sandbox exercise the descriptor
  branch of assertion A's resolution with no binary present. The baseline case is
  the parity oracle for the branch set: it greens only if all four coverage
  branches and all three cycle-valve branches clear, so a branch that stops firing
  reds rather than passing vacuously.
- Assertions **H and I** keep their own hermetic drivers
  <!-- prose-enum-exempt: this member's own driver set, not a subset of the kit's gate-test roster -->
  (`check-graph-refs.test.sh`, `check-graph-cap.test.sh`), and the **theme seam**
  keeps `check-graph-theme.test.sh`. Assertion F is reachable only through a
  theme, because the kit's own emission carries no local `href`/`src` at all.
- A bespoke driver reaches the gate through `gate_run` and never by script path
  (§run-gate-tests), which is what let this member's behavioral tests survive its
  port; what each driver owed at the cut was the **two distinct migrations** the
  port carried — onto `gate_run`, and off the retired theme seam onto part files.

**The live-tree parity arm is demoted from proof to smoke**, per criterion 4's
own rule for a member whose assertion target is gate source: assertion A forbids
a descriptor and a script coexisting in one resolve dir, so the cross-substrate
comparison necessarily ran on the pre-descriptor tree, which the port then
changed. Its verdict is recorded as **no disagreement found on the pre-descriptor
tree**, never as parity proved. What stands in its place is an executed
byte-identity acceptance: with the descriptor's couples widening held out, the
ported emitter over the migrated part files reproduced the pre-port shell
emission byte for byte.

### check-reads-couples

Invariant: for every registered gate, every **statically resolvable recursive
walk** in its source has its tracked read-set covered by the gate's expanded
`couples=` — the reads⊆couples half `check-graph` leaves to the author
(§The `# graph:` manifest).

**A member resolving to a `.gate` is answered by the substrate, not parsed.** Its
walk parser reads shell, so a binary gate yields zero walks and the gate would
print `clean` — the single worst vacuity available at the substrate seam, because
the absence of findings is indistinguishable from an absence of coverage. The way
past that is to give the gate an answer, never to give the port an exemption: for a
`.gate` member the gate reads the declared roots the `--reads` arm reports and
feeds them into the same coverage assertion the shell arm uses. Where that read
comes from is a property of the reader's own substrate: a shell reader invokes
`"$GATE_SDK_NATIVE_BIN" --reads "<name>"` (§Layout and configuration), and the
compiled reader calls the registry in process (below).

**The `--reads` report.** One line per walk root and nothing else — no count line
and no header, because the count is derivable from the lines and a transcribed
total would be a second source for it. Each line is either a repo-relative
directory path, for a root the gate declares, or a single `?`, for a root it cannot
bound statically. Both line kinds have a named reader at a named transition: a path
is read by the tracked-file enumeration below, at the per-root coverage loop; a `?`
by the skip counter, at the clean-line parenthetical — the same reader the shell
arm's unresolvable roots already have. There is no third line kind because there is
no third reader. The producer is the binary's `--reads` arm, printing the declared
roots each registry member carries — data held to what the code actually walks by
two crate-side unit tests (§Meta-gate conservation for the binary substrate).

**A root line may carry one optional field: a tab and the name of the knob whose
value is that walk's `-name` pattern** (`<root><TAB><knob-name>`). A bare root
keeps its present meaning, unfiltered, so every existing declaration is unchanged.
The field's producer is the same `--reads` arm printing the same registry data; its
**only** consumer is this gate's consumption path, at exactly one transition — the
per-root coverage assertion — where it resolves the named knob through the config
bridge it already sources and forwards the resolved value where an unfiltered root
passes the empty pattern. Resolution failure is **fail-closed**: a named knob the
owning kit does not define is exit 2 naming it, never an empty filter silently
widening the demand to the whole root.

**Two alternatives are refused, recorded so a later port does not retry them.**
Declaring `?` for such a root is refused: `?` marks a root that cannot be
*bounded statically*, and every member carrying it does so because its root is an
argument with a default, where these are literal — spelling one `?` would be the
foreclosed opt-out moved into the registry. Re-implementing the scan over
`git ls-files` to fall outside the analyzed class is refused for the same reason
with a behavioral cost on top: enumeration is out of scope *because* it is not a
walk, so using it to evade the assertion is opting out spelled in code, and it
silently narrows the scan to **tracked** files where the walk sees an untracked
one too.

**The filter is carried as a knob name and never as a literal pattern, and that is
the load-bearing detail.** The walks this exists for select by *knob values* — which
is exactly why the shell analyzer extracts nothing for them, since it discards a
pattern containing `$`, so the shell member's exemption was the only channel it had
rather than laziness. Spelling the pattern into the crate's registry to make the
field static would be a second spelling of a knob's default, which
de-literalization forbids. Carrying the name keeps the value single-sourced and
reuses the resolution path the bridge already owns.

**The coverage assertion itself is unchanged; only the source of the roots
differs** — a shell parse for a `.sh` member, the substrate's own report for a
`.gate` one. Two calibrations follow from what a binary does *not* report, and both
err toward demanding more coverage rather than less. A reported root is filtered by
the prune list exactly as a `gate_find` walk is, because the crate's single
sanctioned walk resolves its set from the same two knobs, `GATE_SDK_PRUNE_DIRS`
and `GATE_SDK_PRUNE_EXTRA_DIRS` (§lib/gate.sh — which is where a port learns that
neither is the spelling a member *declares*: the bridgeable one is the resolved
`GATE_PRUNE_DIRS`) — both, because a substrate
honoring only one of an additive pair would scan a different tree than the shell
for any consumer who set the other. And no literal `-name` primary is extractable
from a binary, so a bare root's enumeration is unfiltered — the same answer the
shell arm already gives a walk whose pattern is a variable. A ported gate narrows
by declaring a tighter root, or by naming the **knob** its pattern comes from
(above); it never declares a pattern.

**The refusal survives, narrowed to the cases where the gate still cannot see.** A
descriptor naming a subcommand the substrate does not carry is exit 2, so an
unavailable read set cannot read as "reads nothing", and an unresolvable filter
knob is exit 2 by the same contract. Both are the §Fail-closed contract applied to
a corpus the gate cannot see: "cannot verify" and "verified covered" must not share
an exit code.

**Ported to the binary substrate at §The sixth budget batch, and the port ends its
own `c7=?` by answering it.** The unresolvable command-position expansion the
blocker report could not adjudicate was the gate binary itself
(§port-blockers), so the compiled member reaches the read set **in process**
rather than spawning the arm — the same adjudication §The fourth budget batch
recorded for `check-gate-binary-fresh`. Three consequences, each stated because
none is visible from the rule above. The in-process reach is a call across crate
modules and therefore a nameable coupling the descriptor carries (§The non-gate
arm). The **absent-or-not-executable-binary** refusal is gone rather than
retired-with-cause: the binary is running by construction, so the condition cannot
arise, and its disappearance is the assertion being answered rather than dropped.
And its fixture harness now names **real registry members** and lets the substrate
answer for them: a stub `--reads` provider is a seam the compiled reader does not
have, so driving one would exercise nothing the gate does.

**The knob a run-time-named filter resolves through is declared by a union
sentinel, ruled 2026-08-19.** The name a filter knob carries is not known until
this member reads another member's declared roots, and the bridge resolves only the
fixed set a member declares. So the member declares, beside its own knobs, a
sentinel the registry expands to **every filter-knob name any member declares** —
computed from the same compile-time tuple the roots themselves are declared in, so
the set is derived rather than maintained and a newly declared filter knob cannot
be forgotten. The expansion is a crate-internal carrier: `--knobs` still prints one
knob name per line and no `.gate` descriptor field moves. Spawning a shell to reach
the kit library instead is refused — it would make an interpreter a *run-time*
dependency of a compiled gate, converting a transitional dependency into a
permanent one against a closed ruling (TRAJECTORY.md §The objectives). The
fallback, if a later consumer's filter knobs outgrow a single union, is the
bridge's **per-kit prefix family**, which resolves cleanly because the prefix
expander matches the remainder against each kit's derived prefix; only an all-kit
wildcard has no owner.

**There is deliberately no descriptor-level exemption.** The live port briefly
shipped one — a `# reads-couples-exempt:` line in the descriptor bought the
member a pass — on the argument that the one ported gate's single walk was
already undecidable, so the exemption converted a counted skip into a counted
exemption and ended no assertion. That was true of that gate and false as a
rule: the *next* port has resolvable walks, and an opt-out written in one
sentence is how a port ends the assertion it was supposed to replace rather
than replacing it. The allowance was also never written here, so the refusal
this section states and the behavior the gate had disagreed — the divergence
that makes "land it then relax" hard to see. Removed with the port, and not
reinstated by the consumption path above: a port ends this assertion by
**answering** it, never by opting out of it.

**The arm has a live instance, and it is the filtered form.** `check-stage-entry`
declares one root twice under two filter-knob names
(lifecycle-kit/SPEC.md §check-stage-entry), so the coverage assertion runs for
real rather than reporting a counted zero — which it did through every cohort
before this one, over a live descriptor set rather than an empty tree. That member
is also where the no-descriptor-exemption ruling below is paid rather than
softened: its shell form carried two `# reads-couples-exempt:` markers, and the
port ends the exemption by **answering** the assertion — the couples widen to
cover what the walks genuinely read.

`gate-tests/check-reads-couples.test.sh` is what keeps the unexercised branches
tested rather than merely reachable, and it is what **proves the mechanism without
a port**.
It drives the consumption path against a stub binary — a reported root whose
tracked reads its couples cover, one whose couples stop a level short, and one
reporting `?` — both surviving refusals, a descriptor still claiming the removed
exemption (which buys it nothing, so the opt-out cannot return unnoticed), and the
shell arm alongside so no case can be passing for a parse failure. One case runs
the **real** binary rather than a stub, so the grammar under test is the one the
substrate actually emits; it is skipped and named in the test's own summary line
when the binary is not built, because the file is hermetic and never builds one.
Naming the skip is what keeps it from reading as a silent pass.

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
it reads as content (every `checks/` dir, `gates.list`, and the crate's source —
an edit to a gate's implementation changes what it walks, so without that couple
the assertion would be live and unreachable, coupled to shell sources only on a
rule whose subject had moved); the tracked-file enumeration is `git ls-files`
metadata, not a content read, so it needs no couple. The crate couple fires **in
this repo**, where the implementation is tracked; a consumer tree receives no
crate source by ruling, so there is no edit for it to catch and a glob matching
nothing is the correct outcome rather than a hole. The hermetic fixture affordance: positional gate-source arguments make
the gate analyze the given source(s) with `git ls-files` anchored to the case
dir, instead of walking the real `gates.list`.

### enforcement-map

The emitter is a **non-gate arm of the gate binary**, `--emit-enforcement-map`
(§The non-gate arm), reached as
`bash gate-sdk/bin/run-gates.sh --emit enforcement-map` because an arm receives
no configuration of its own and that front-end resolves its bridged knobs first.
It writes `docs/enforcement.md`: a kit-first map of
every check surface — kit, governed surface, enforcement class — derived from
the class registries so it cannot drift from what actually runs. It is
check-graph's sibling in shape (an emitter whose output a freshness gate
byte-compares), advisory by construction: it never joins `gates.list`, and a
*healthy* run exits 0 whatever registries are absent — while a misconfigured run
exits 2, fail-closed, leaving an empty projection rather than a plausible partial
page that would byte-match itself on the next freshness check.

**Where the fail-closed decision now sits has moved, and the move is the
substance rather than a relocation of wording.** The shell form captured each
registry knob's *set-ness* before defaulting it, so it could tell an
explicitly-misconfigured registry from an unadopted one. A bridged knob crosses
as a value with no set-ness attached, so each registry's owning library now makes
that call at resolution time — `DRIFT_KIT_KPIS_FILE` resolving empty for
not-adopted (drift-kit/SPEC.md §lib/drift.sh), `CONTEXT_KIT_SETTINGS_FILE`
refusing on an explicitly-set missing path (context-kit/SPEC.md §Layout and
configuration) — and what remains here is the reader's own half: a registry that
resolves to nothing drops its section, and a roster naming a member the bridge
did not carry refuses naming it.

The emission is a **library function** the arm wraps rather than the arm itself,
which is what lets §check-enforcement-fresh call it in-process and the value
rollup consume the class taxonomy and per-kit counts as data instead of
re-parsing this page's headings.

Each enforcement class reads one registry, every registry defaulting to this
repo's layout through the owning kit's knob: **blocking gates** from
`gates.list` plus each gate's `# graph:` `tier=` field, the owning kit taken
from the same name-resolution walk the runner uses (a consumer-dir gate groups
as the consumer's); **advisory KPIs** from the drift-kit `kpis.list` registry
(`DRIFT_KIT_KPIS_FILE`); **guards** and **session warnings** from the
`PreToolUse` / `SessionStart` command hooks in the tracked harness settings file
(`CONTEXT_KIT_SETTINGS_FILE`, parsed with the crate's own JSON reader) — a guard row's *intercepts* cell
carries the harness matcher verbatim, so its `|` alternation is escaped on the way
into the table, a case a freshness gate can never report because it compares the
emitter against the page and both would carry the same broken row; **validate
suites** from
evidence-kit's suite registry — the roster from `EVIDENCE_KIT_SUITES` and each
suite's run command looked up **by name** in the bridged `EVIDENCE_KIT_RUN_`
family, never enumerated, since a prefix is a resolution set and not a roster
(§lib/gate.sh); and **monitors** — the
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

Invariant: `docs/enforcement.md` byte-matches the enforcement-map emitter — the
check-graph / trajectory-freshness byte-compare pattern. Bare, it runs the
emitter and compares the committed page; given two arguments
(`projection-file emit-file`) it compares pre-baked files, letting the fixture
pair exercise it hermetically off the live registries. Fail-closed: a missing
projection, a missing emit source, or a failed emit is a red (exit 2), never a
false clean.

**It is a registry member of the gate binary, and its emitter is a function call
rather than a spawn.** The comparator and the emitter ported in one unit, so
where the shell form ran `bash <emitter> --emit` in a subprocess, the compiled
member calls the emitter module's `emit()` **in-process** — retiring the
family's `bash` hop for this member as it did for §check-footprint-fresh. Every
registry knob the emitter reads is declared by this member and arrives across the
config bridge, including the `EVIDENCE_KIT_RUN_` **prefix family**; because the
hermetic two-argument mode bypasses the emitter entirely, those knobs are
resolved but unread in a fixture run, which is why an empty family must resolve
rather than refuse (§lib/gate.sh). Its `# graph:` manifest couples every class registry — the gate
sources (so a `tier=` edit re-fires), `kpis.list`, the settings file, and the
monitor-carrier workflows — beside the artifact itself, so any registry change
re-runs the freshness compare. The corrective its help text names is the
regeneration command, reachable because the gate rides the generated pre-commit
hook (the check-graph contract exactly).

### check-kit-enum

`checks/check-kit-enum.gate` (`precommit`, binary-dispatched).
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

**The port made one ordering deterministic, and it was never specified** (§The
fourth budget batch). The shell form reported a member's glob groups in bash's
associative-array hash order — so a
member violating on two globs at once listed its findings in an order neither
stable across bash builds nor stated anywhere. The compiled form keeps
first-seen order, which is the manifest's own. The verdict is set-valued either
way; only a multi-group report's line order was ever at stake.

**Criterion 4 binds here through the walk *and* through the trigger**, which is
what made its fixture pre-work real: the gate resolves every registered member's
declaration and reads its manifest as text, so its own declaration path is inside
its corpus. Neither case resolved a member to a `.gate` before the port, and the
multi-kit hand-list branch — the whole rule — fired on no live group, so the pair
proved the parse and not the assertion. Both cases now carry a descriptor
declaring a complete two-root group, and the good case asserts the group count.

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
(`[registry-doc [runner-doc]]`) overrides. **Both defaults are resolved in
`lib/gate.sh`, not inline here**, because the config bridge refuses a knob the
owning kit's library does not define (§lib/gate.sh) — an inline default is
invisible to it, so a compiled member declaring either knob would fail-close on
every invocation. Fail-closed: a configured doc that
does not exist is a misconfiguration (exit 2, like `check-kit-enum`'s missing
registry), as is a non-repo cwd or empty roster — never a false clean. A
consumer keeping no prose registry opts out by not registering the gate in its
`gates.list`; there is no empty-knob valve. This gate retires close's manual
"does the kit table still reflect the kit set?" staleness check, narrowing that
step to the un-gated remainder (row descriptions, per-kit READMEs).

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-kit-registration.gate`, rule out of the gate
binary, proved parity-identical before the shell gate was deleted
(§The kit-roots `gate_kit_roots` cohort). A **wrapper** re-scoping this
invariant onto another registry doc reaches it through `gate_command`'s argv
rather than by executing a path, which is the only spelling that survives the
move; this consumer's `check-docs-kit-parity` is the live instance.

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
`check-readme-roster [root]` resolves relative kit roots against a fixture
tree (the case dir's `gate-sdk-config.sh` names the fixture kits), the sibling
meta-gates' hermetic-fixture shape; bare, it sweeps against the git toplevel.
Fail-closed: a non-repo cwd with no root argument, an empty roster, or an
unreadable README marker scan is exit 2, never a false clean.

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-readme-roster.gate`, rule out of the gate
binary, proved parity-identical before the shell gate was deleted (§The second
budget batch, where the criterion-4 fixture widening that had to precede that
proof is recorded).

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
no new knob. Positional form `check-smoke-entry-guard [root]` resolves
relative kit roots against a fixture tree (the case dir's `gate-sdk-config.sh`
names the fixture kits); bare, it sweeps against the git toplevel. Fail-closed:
a non-repo cwd with no root argument, an empty roster, or an unreadable smoke
script is exit 2, never a false clean. The `# graph:` couples the mutating
smoke scripts (`kit:smoke/install.sh,kit:smoke/violation.sh`, `dir=one`,
`tier=precommit`), so editing one re-fires the gate.

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-smoke-entry-guard.gate`, rule out of the gate
binary, proved parity-identical before the shell gate was deleted
(§The kit-roots `gate_kit_roots` cohort).

### check-core-files

`checks/check-core-files.gate` (`precommit`, binary-dispatched).
Invariant: every path in the consumer's `core-files.list` manifest exists in
the worktree **and** is tracked (`git ls-files --error-unmatch`). Red on a
missing or untracked listed path — one existence-plus-tracked test catches a
plain `rm`, a `git rm`, and a listed-but-never-added path alike, with no
`--diff-filter` timing window that only sees the loss at some later stage.

The manifest is optional consumer config (the `graph-vocab.sh` pattern): the
path knob is `GATE_SDK_CORE_FILES_FILE` (default
`<gates-dir>/core-files.list`, resolved onto the knob's own name in `lib/gate.sh`
so the config bridge's `declare -p` can find it), registry-style — one repo-relative path per
line **or** a `kit:<path>` token, `#` comments and blanks ignored. An absent
manifest is clean with a note;
an empty or comment-only manifest is clean; a present-but-unreadable manifest
is fail-closed (exit 2). Calibration: the intentional-removal valve is the
manifest itself — retiring a surface means deleting its line in the same commit
that removes the file, a diff-visible edit that needs no exemption tag, so the
gate is re-scoped in the open, never weakened to pass. The bespoke
`gate-tests/check-core-files.test.sh` carries the expansion, the wildcard
refusal and the untracked-but-present branch — none of them expressible in the
pair, since the fixture runner reads exit 2 as a harness error and runs each
case with the fixture dir as cwd — and it dispatches through `gate_run`, which
is what keeps a bespoke test alive across a substrate move
(§lib/test-hermetic.sh).

A `kit:<path>` line derives one `<kit-root>/<path>` entry per `gate_kit_roots`
member — the same expansion, the same root
set, and the same spelling `# graph:` `couples=` fields already use (§check-graph
owns them; nothing about them is re-specified here). What is new is only that
`check-core-files` is a second reader of it. A manifest with no `kit:` line
carries no derivation at all, so it needs no root set and reads none.

**The compiled form derives it from the bridged root set rather than calling the
shell expander**, which is criterion 6's discharge-by-construction and not a
second implementation of the token: `GATE_KIT_ROOTS_REL` is the *resolved* value
of the one derivation `lib/gate.sh` owns (§lib/gate.sh), so the root set is
computed in exactly one place and the binary holds no default to drift from. What
the crate carries is the join and the wildcard refusal — this reader's own rule,
which the shared expander deliberately does not have.

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

`checks/check-identity.gate` (`precommit`, binary-dispatched).

Invariant: every expectation in the `identity.conf` manifest matches this
clone's identity — a verification backstop for the fresh-clone gap
where an agent commits or pushes under the wrong identity and fails silently
(misattribution is unpurgeable without a SHA-breaking history rewrite; the
wrong-SSH-key symptom is a misleading "Repository not found"). Multi-identity
setups — a work and a personal account on one machine — make this the common
case for the integrator audience. **Scope fence:** the identity *mapping* stays
git's and the GitHub CLI's job (`includeIf`, `core.sshCommand`, the CLI's own
account switch); this gate only asserts the mapping actually applied here.

Three expectation kinds, all local reads (cheap, no network, no false positives
from a settled corpus):

- `email <expected>` — matches `git config user.email` exactly.
- `remote-host <remote> <host>` — matches `<host>` against the host part of
  `git remote get-url <remote>` by exact string. An SSH host alias is matched
  as the alias — that *is* the identity selector in multi-identity setups, so a
  scp-like `git@alias:path` compares as `alias`, and a `scheme://[user@]host/…`
  URL as `host`. A configured remote that does not exist in this clone is red.
- `gh-account <login>` — exactly two fields, matched against the GitHub CLI's
  **persisted** active account for the configured host by exact string. The
  hazard it closes is not the one the first two close: `user.email` and a
  remote's host are per-clone state, so a wrong value is wrong only where it is
  set, while the CLI's active account is **machine-global** — one project
  switching it repoints every CLI call in every other clone on the box, the
  credential helper a push rides included, and the symptom surfaces as a
  permission error against the wrong repo rather than as anything naming the
  account. A consumer project cannot notice its account moved underneath it.

The manifest is optional consumer config (the `graph-vocab.sh` pattern): the
path knob is `GATE_SDK_IDENTITY_FILE` (default `<gates-dir>/identity.conf`),
line-based `key value…` with `#` comments and blanks ignored. An absent, empty,
or comment-only manifest is clean with a note; a mismatch (or a manifest-named
remote that is absent) is a violation (exit 1); a malformed line — an unknown
key or wrong field count — is fail-closed (exit 2), never a false clean on an
uninterpretable manifest. **Every one of those contracts reaches the account
kind unchanged, and that is the whole argument for putting the kind here rather
than in a gate of its own**: a wrong-field-count `gh-account` line — a
three-field spelling carrying a host, say — is fail-closed by the arm that
already exists, not by a new one, and adding the key *narrows* the unknown-key
arm's corpus by exactly one token, which can only remove violations from it.
A run under CI (the vendor-neutral `CI` env var)
is clean-skipped ahead of the manifest reads: the server-side battery is not a
committing clone, so there is no local identity to misattribute a commit or
push with, and the CI runner's unset `user.email` is expected, not a violation.
**The step-aside binds only where an actual is still read from the clone** — with
both actual-source knobs below configured, the run is a configured read rather
than a self-check of a committing clone, so it proceeds and the pair stays
deterministic wherever it runs. The same scoping covers the
is-this-a-git-repository precondition, for the same reason.
Enforcement is dual: the `# graph:` couples the
manifest at `tier=precommit` (a `git config` change to the mapping is not
diff-visible — nor is a CLI account switch, whose hosts file is machine-global,
outside the tree and never stages, so no coupling could name it either — so the
whole-tree `run-gates.sh` battery is the real backstop for
the commit-identity half), and `install-hooks.sh` runs the gate once at opt-in
to cover the push-identity half (no pre-push hook is added — gate-sdk generates
only the pre-commit hook, and the setup rung plus the precommit tier already
cover the surface). That rung reaches the gate through `gate_command` rather
than by interpreting the resolved declaration path (§install-hooks).

**The clone's actuals are redirected by knob, and the `[manifest]` positional and
`--fixture <dir>` arm are retired.** The positional overrode
`GATE_SDK_IDENTITY_FILE` and nothing else, so a knob already redirected every
path it redirected. The arm did not clear that ground as written: it redirected
*three* things and only the manifest had a knob, the other two being what the
clone itself says. So the two missing knobs are minted rather than the arm
deleted on a ground that does not hold — `GATE_SDK_GIT_EMAIL_FILE` and
`GATE_SDK_GIT_REMOTES_FILE`, each naming a file standing in for one actual, each
empty by default so the gate falls through to the live `git` read, which is the
production path. `GATE_SDK_GH_HOSTS_FILE` is the family's third member, on the
same shape and with the one genuine **live** use among them — a relocated CLI
config — so the account kind's test path and its production path are the same
path, which is the property the arm's deletion exists to buy. Reaching
the gate through knobs is what makes the fixture pair a parity oracle **for the
live arm** instead of for a fixture-only second code path, which is the payoff
context-kit/SPEC.md §check-memory-off states for the identical deletion.

**The honest limit, since the knobs are new capability and not only test
plumbing.** A knob redirecting what the clone's identity *is* can be set to make
the gate agree with a manifest it should disagree with. That is not a weakening:
the manifest is local config the same operator writes, the gate is a self-check
rather than a security boundary, and the scope fence above already puts
*performing* the mapping outside it. Stated so the capability is a ruling rather
than a side effect.

**The account kind's oracle is a local read of the CLI's persisted config, and
the obvious oracle is refused with cause.** The CLI's status subcommand is the
one a later author reaches for and it validates the token over the network; at
`tier=precommit` that reddens an offline commit and puts a network round-trip in
the pre-commit path, so it is refused. What the gate reads instead is the CLI's
own hosts file — the file mapping each host to that host's credentials and to
the active account — resolved through `GATE_SDK_GH_HOSTS_FILE`, whose default
derivation honours the CLI's own config-dir variable first and the XDG config
home second: a consumer that has relocated its CLI config has already said so
once, and re-asking it through a kit knob would be a second source for one fact.
**That derivation lives in the member rather than in `lib/gate.sh`**, which is
the departure from the knob-defaults rule and has a cause: it reads `$HOME`, and
a `HOME`-less derivation would yield a path under `/` that is absent — which the
grading below reads as *clean*, the one false clean this kind exists to refuse.
So the knob's shell default is empty, meaning *derive it* rather than *no file*,
and a derivation with nothing to stand on is exit 2. Both halves are
context-kit/SPEC.md §check-memory-off's, which takes the same refusal on the
same variable.

**The parse, and the key collision that decides it.** Inside the block
introduced by the configured host, the active account is the value of the key
spelled exactly `user`, at that block's own key indent. The sibling key `users`
— the map enumerating the accounts *available* on this machine — shares that
prefix and the CLI writes it **first**, so a substring or startswith match hits
the map header before the active-account key and reads a structural key as a
login; the indent scoping additionally keeps a login literally spelled `user`
*inside* that map out of reach. Recorded here rather than left to the
implementation because it is the defect a reasonable implementation walks
straight into, and a fixture pair cannot fail on a case nobody thought to write.

**The version tolerance, which is what makes binding to a tool's internal config
format defensible.** The gate binds to `user` and deliberately does **not** read
the `users` map: the map answers *which accounts exist here*, a different
question from *which one is active*, so a design reading it would answer the
wrong question even where it parsed cleanly. A file whose shape yields no `user`
key for the configured host is **exit 2** — an unrecognized shape is fail-closed,
never a clean, which is the one posture that keeps a format change from silently
retiring the assertion.

**The host is a knob, not a manifest field.** `GATE_SDK_GH_HOST` (default
`github.com`) names the host whose block is read — config-via-env, mirroring the
CLI's own host variable. The alternative is refused with cause, because it too
is one a later author will reach for: a three-field `gh-account <host> <login>`
would let one manifest pin two hosts, and it would move the field count the
fail-closed arm keys on. A consumer pinning two hosts from one clone is not in
evidence; a consumer on a single enterprise host is served by the knob.

**The absence posture is graded rather than binary** — three conditions, three
verdicts:

- the hosts file is **absent** — clean, with the fail-open caveat named *in the
  clean line*, and the unverified expectation excluded from the count that line
  reports. A clone with no CLI cannot push through it, so the hazard this kind
  guards cannot arise there; context-kit/SPEC.md §check-memory-off is the
  precedent and the shape is taken from it verbatim.
- the file is **present and cannot be read or parsed** — exit 2. The surface
  exists and the gate cannot say what it holds; a clean there is a false clean on
  the one condition the kind exists to catch. **No probe silences stderr**: a
  suppressed read error would turn an unreadable file into an absent one and
  collapse this verdict into the one above, which is the exact false negative the
  grading exists to keep apart.
- the file is present and carries **no block for the configured host** — a
  violation, exit 1. The manifest says the account should be one thing and this
  machine is not logged in to that host at all.

**The distinguishing principle, stated so the third does not read as
inconsistent with the first.** This gate already reds when a manifest-named
remote is absent from the clone, and that is not in tension with
clean-on-absent-CLI: a remote is **repo-local state a manifest may demand**,
while the CLI is **machine state outside the repo** that no repo-local manifest
can require to be installed. Absence of the tool is outside the manifest's
authority; absence of a login *within* a configured tool is inside it.

**The subject is the persisted account, not the effective one**, and that
wording is load-bearing rather than incidental: a token environment variable
makes the CLI authenticate as that token's account without touching the hosts
file, so an assertion worded over "the account the CLI would use right now"
would be false wherever one is set. The honest limit rides with the ruling and is
small on purpose: a per-process token override is not detected, and it is also
not the hazard — the recurrence this kind exists for is a *persistent* switch
left behind by a sanctioned release action, which damages every other clone on
the box, while a token in one process's environment persists nothing and reaches
no sibling. Scoping the assertion to the persisted state narrows it onto the
thing that actually recurs rather than conceding a gap.

**The seam: the kit ships the kind, never an account.** A login is one
project's — one *person's* — vocabulary, and a crate constant holding one would
publish it as everyone's mechanism. The manifest is already optional consumer
config on the `graph-vocab.sh` pattern, so the kind inherits the discharge: the
crate carries the key name, the parse and the comparison; the consumer's
manifest carries the login; the host is a knob with a generic default. This
repo's own `scripts/identity.conf` deliberately carries **no** `gh-account`
line, so checkwright stays undetected by its own gate and ships mechanism a
consumer opts into. What that costs is stated rather than discovered: **no live
run in this tree ever executes the kind**, so the fixture pair plus
`check-identity.test.sh` are its whole oracle, and the sibling is where the
graded postures above live — a one-pair harness can hold the match/mismatch axis
and nothing else.

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

`checks/check-exec-bit.gate` (`precommit`, binary-dispatched).
Invariant: every tracked `*.sh` path matching an exec-glob carries git *index*
mode `100755`, **and every tracked `*.gate` descriptor carries `100644`**. The
second is stated as an assertion rather than left implicit so that "a descriptor
is not executable" cannot read as "a descriptor is not covered": the descriptor
is data — a manifest and directives, never sourced and never run — and an
executable one invites a reader to run a file carrying no interpreter line. The
first class is by-path-invoked kit scripts — gate-sdk's runner
(`run-gates.sh`), drift-kit's overhead meter (`overhead-meter.sh`), and lifecycle-kit's
entry preflight all invoke kit scripts **by path**, and a shebang'd `bin/` tool
is by-convention path-invocable — so a script committed `100644` degrades
silently in a fresh clone: a KPI plugin to `n/a (plugin failed)`, a
runner-invoked preflight to a skipped check. The index is the checked surface
because it is the mode a clone receives, and a `Write`-tool-authored script
acquires `100644` there regardless of worktree state; one `git ls-files -s`
reads it, sidestepping the worktree bit.

The subject class is two knobs (both join §Layout and configuration's roster),
each resolved in `lib/gate.sh` to an array under a distinct name — `GATE_EXEC_GLOBS`
and `GATE_EXEC_PRUNE` — which is what the bridge carries and what the compiled
member declares. That spelling is the one §lib/gate.sh's naming rule grants: a
whitespace scalar feeding an array would otherwise be one name meaning two
grammars, the cause `GATE_PRUNE_DIRS` already exists for.
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

Argument mode (fixture capability): `check-exec-bit [ls-files-dump]` lints a
canned `git ls-files -s` dump instead of running `git ls-files -s` from the
repo root, so a fixture is hermetic against the host repo's index (the
check-merge-attrs precedent).

**The two classes are reported one at a time, and that is what shapes the
pair.** A dump violating both prints the script class and exits, so the
descriptor class cannot appear in the same case. The `bad/` case therefore
carries the *descriptor* violation — the class no case reached before the port,
under a rule this batch landed descriptors beneath — and the script class keeps
its own case in the bespoke `gate-tests/check-exec-bit.test.sh`, which builds a
temp git repo and re-stages a KPI at `100644` then `100755` on the live
`git ls-files` path. That test invokes the member through `gate_run` rather than
by script path, which is what keeps it alive across a port
(§lib/test-hermetic.sh; §The third budget batch records the class of failure a
by-path invocation produces).

### check-root-tiering

`checks/check-root-tiering.gate` (hermetic, `precommit`, binary-dispatched).
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

**Its implementation is a compiled subcommand.** The declaration path is
`check-action-pinning.gate` — a `.gate` descriptor carrying the manifest above and
nothing else (§The `# graph:` manifest); the rule runs out of the gate binary,
proved byte-identical to the shell gate it replaced before that gate was deleted
(§The first cohort, and the rule that selects the next). Everything this section
specifies is substrate-independent and was written against the shell
implementation, which is the point: the seam carries a gate's rule without
changing its verdict, so nothing above is restated per substrate.

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

**Scan set — derived, then narrowed to the subject.** Stage one is a pruned walk
for `*.yml` / `*.yaml` from the scan root (the optional positional argument,
default `.`), taking the shared prune set, so `gate-tests/` is out and the `bad/`
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
earns its place at a second consumer and **this extractor** has none. The clause
is scoped rather than absolute, because the tree now holds one of each answer:
§check-action-gh-repo's job-partitioned walk *did* find its second consumer and
moved to `native/src/actions.rs` under it (§check-action-permissions), while this
extractor is deliberately not that walk's third consumer — the two share an
indentation convention, not a mechanism, and §check-action-gh-repo carries the
reasoning for both directions. Its rules, each of which a prototype proved
necessary by failing without it:

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
- **The job partition.** A column-0 key `jobs` opens the job section and any
  other column-0 key closes it — the Actions-shape predicate's own test, reused
  rather than re-spelled, since a `jobs:` key at any other indent is a foreign
  schema the predicate already refuses to see. The first indent under `jobs:` is
  the **job column**: a line at it begins a job, ends the job before it, and
  resets that job's `runs-on` to *absent*. The first indent under a job id is the
  **job-key column**, where a `runs-on:` key captures the job's runner value; a
  value that is empty on the key line takes the following lines more indented
  than that column, so a block sequence and a mapping are captured whole rather
  than read as absent. A trailing YAML comment is stripped from the captured
  value, so a label carrying one is still matched by the Windows test below.
- **A `runs:`-shaped composite action never enters the job section**, so its
  steps carry no job and no `runs-on`. That is correct rather than a gap:
  GitHub's own schema makes `shell:` **required** on a composite `run:` step, so
  a composite step reaching the unresolved row of the dialect table is invalid
  Actions YAML and the finding is true.

**Resolution happens at two boundaries, and the split is what makes the second
axis derivable.** The step boundary stamps the step's **explicit** `shell:` onto
its blocks, because that sibling key may sit either side of the `run:` block it
governs. A job's `runs-on` has the same freedom against the whole `steps:` list —
YAML admits it after `steps:` — so the *inferred* half cannot be resolved at the
step boundary without reading a key that may not have arrived. A block therefore
carries its step's explicit value as an **optional**, and the job boundary (or
the file's end) stamps the job's `runs-on` class onto every block that has none.

**GitHub expressions.** `${{ … }}` is not shell syntax; left raw it is a parse
error. It is replaced per line by `${GHEXPR}`, a braced parameter expansion,
which presents to ShellCheck as the opaque runtime value a GitHub expression
actually is. A bare word does not work and the difference is measured, not
stylistic: a literal constant drags ShellCheck's constant-expression analysis
into firing on correct code, manufacturing SC2050 inside `[ … ]` and SC2194
inside `case`. The braced form causes no finding in any tested position.

**Dialect — resolved, never assumed, on two axes.** Linting a block under the
wrong dialect manufactures false positives, so the step's effective shell comes
from its `shell:` sibling key where it has one and from the enclosing job's
`runs-on` where it does not:

| `shell:` | `runs-on` | resolution |
| --- | --- | --- |
| absent | non-Windows | `-s bash` — GitHub's documented default for a `run:` step everywhere but a Windows runner |
| absent | Windows | **finding** — the runner's default is `pwsh` and the step does not say so |
| absent | unreadable, or the step has no enclosing job | **finding** — the dialect cannot be stated, so it is not assumed |
| `bash` (with or without arguments) | any | `-s bash` |
| `sh` / `dash` / `ksh` | any | the matching ShellCheck dialect — linting a POSIX body as bash hides the portability findings that dialect exists to surface |
| anything else (`pwsh`, `python`, a custom `{0}` template) | any | the block is **skipped and counted** — the body is not shell, so there is no shell to lint |

**One rule covers both new rows: a step's dialect must be knowable, and where the
gate cannot state it the step says it.** The finding line names the file, the
`run:` block's line and which condition fired; the class takes its own `help:`
line (§Output contract's rule for a gate with more than one failure class), and
the remedy it names is one line — `shell: bash`, or `shell: pwsh` where pwsh is
what was meant.

**`runs-on` is classified by a Windows test, never by a platform roster.** The
captured value resolves to one of three classes:

- **Labels.** A scalar value is one label. A sequence — flow (`[a, b]`) or block
  — is its members. A mapping is a runner group: its `labels:` members are the
  labels, and a mapping carrying `group:` with no `labels:` yields none.
- **Windows** if any label, lowercased and stripped of surrounding quotes, is
  exactly `windows` or begins `windows-`. *Any* label matching makes the job
  Windows: a mixed self-hosted selector may land on a Windows machine, and a
  dialect that is only *probably* bash is not one the gate may state.
- **Unreadable** if the captured text contains `${{` anywhere, or if the value
  yields no labels at all — the group-only mapping, an empty value, an absent
  `runs-on`.
- **Non-Windows** otherwise, and that is the *only* other answer. There is no
  Linux label set and no macOS label set, because bash is GitHub's default
  everywhere that is not Windows: enumerating the platforms that resolve to bash
  would be a maintained roster of runner labels drifting against a provider's
  release notes, which derivation-first refuses. The gate asserts one distinction
  because one distinction is what the dialect turns on.

**The honest limit, stated rather than papered over.** A self-hosted Windows
runner registered without a `windows` label reads as non-Windows and its
unshelled bodies are linted as bash. The label convention is the only platform
signal a tree-local reader has, and widening the match by guessing at label
vocabularies is what turns a stated assertion back into a heuristic. The remedy
in that tree is the same one line the finding asks for.

**Why the Windows row is a finding and not a skip.** Skipping would make the
inferred case agree with the explicit `pwsh` row, and that disagreement was the
original complaint — the resolver taking the opposite branch from the one it
takes on the same body spelled out. But agreement is not the objective: nothing
was red under the assumed resolution *because of a habit rather than a
mechanism*, `.github/workflows/gates.yml`'s Windows job naming `shell: bash` on
every one of its steps with a header comment saying why. A skip leaves that habit
a habit and adds a silent lint hole on exactly the platform this tree is buying.
A finding converts the header comment into the mechanism it describes, at a cost
of one line on a Windows step — and a Windows author who genuinely wants pwsh
writes `shell: pwsh` and is skipped-and-counted through the row that already
existed.

**Why the unreadable row is a finding and not a refusal.** Exit 2 is for a
construct the gate cannot process; here the gate processed the file perfectly and
found a step whose dialect nothing in the tree states. That is a property of the
workflow, which is exit 1 — and unlike a refusal it names a remedy the author can
take. §Fail-closed contract is untouched: nothing is captured and read as clean.
The class is not hypothetical and its in-tree instance is the one that mattered:
`.github/workflows/publish.yml`'s `build` job is `runs-on: ${{ matrix.runner }}`,
resolved at runtime from the roster job's hand-kept runner map, so on the day a
Windows target joins `native/targets.list` that leg's bash bodies would run under
`pwsh` while this gate reported them clean. Its step names `shell: bash`, which is
both the line that clears the finding and the line that keeps the release working.

**Its implementation is a compiled subcommand, and it is criterion 7's second
landed wrapper.** The declaration path is `check-action-run-shell.gate` and the
rule runs out of the gate binary. The rule invokes `shellcheck`, a program the
payload does not carry, so criterion 7 reports it (§The port-candidate criteria),
and under the 2026-08-23 ruling recorded there the compiled form spawns the same
program and refuses at exit 2 when it is absent — the requirement this shell form
already placed on every consumer registering the gate, moved by the port not at
all. The wrapper mechanisms are §Fail-closed contract's, established for the
class at §check-shellcheck's port and consumed here rather than rebuilt: the
presence probe firing where the shell form's `command -v` fired it, and the
merged capture whose exit code this member reads to tell a finding (1) from a
fragment ShellCheck could not lint at all (≥2).

**The extractor is the port's content, and its two standing rulings moved with
it.** The *stays inline* rule and the *skipped and counted, never linted as
shell* dialect rule are locality-class directives binding to a line of
implementation, so both follow the code into the crate module (§The `# graph:`
manifest, the annotation partition) rather than into the descriptor.
§check-action-gh-repo's finding that it is not the second consumer the inline
rule waits for is what keeps that rule true across the substrate change.

**Criterion 4 binds, and the live-tree arm is undemoted.** The pre-port rule was
restored under a non-resolving name inside the resolve dir (§The port-candidate
criteria, criterion 4) and both forms were driven from the same cwd with the same
argv over thirteen arms — the live tree, both fixture cases, a YAML-free tree, a
refusal case, an out-of-subject case and an absent scan root, each with the
linter present and again with PATH scrubbed of it. All thirteen matched byte for
byte including exit codes. Unlike §check-shellcheck's arm the probe file carries
no bound: this member's corpus is `*.yml`/`*.yaml`, so a restored `.sh` sits
outside the corpus it probes and the comparison is over the committed tree.

**Two refusal-messaging arms are retired by the port, both unreachable by
construction.** Each existed only to grade a child process and name it in a
message with its exit status. The compiled extractor returns its own refusal and
the walker carries its own error text, the shape §check-action-pinning's port
already took, and there is no child left to name. The refusals themselves
survive: an unreadable corpus is still exit 2.

**One deliberate difference, asserted rather than normalised away**, in the shape
§The first cohort's sort-order note set: the shell walked its corpus in `find(1)`
order, a filesystem artifact, and the compiled walk sorts. A multi-file finding
list therefore comes out in a different — and now deterministic — order. The
counts on the clean line are order-independent, so nothing else moves.

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
value, since no anchor resolution is attempted; an unbalanced `${{` on a body
line; and a `defaults:` key at column 0 or at the job-key column whose subtree
carries a `run:` key. That last one is the dialect resolution's own refusal:
`defaults.run.shell` overrides the runner default for every step beneath it, at
workflow or job level, so a resolver reading `runs-on` and not `defaults` would
answer bash for a job whose steps all run under pwsh — the wrong-dialect lint
this gate exists to end, reintroduced one level up. Modelling it properly means a
third inheritance layer with its own precedence rules; refusing it costs nothing
and can never become a false negative, which is this section's own stated reason
for preferring a loud refusal. **The cost is measured at zero**: a tree-wide grep
for `defaults:` in every `.yml`/`.yaml` returns exactly one hit, `docs/_config.yml`,
which is Jekyll's and is not Actions-shaped — no workflow, no kit template and no
fixture carries one. The refusal is the `run:` key rather than the `defaults:`
key: a `defaults:` block setting anything else changes no step's dialect and is
extracted normally. Refusing the folded form makes the literal form a conformance requirement
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
shared one, and the severity is the family's literal. **No valve either, and the
refusal is deliberate:** the `# graph:` manifest keeps `valve=none`, because
every finding the unstated-dialect class raises is discharged by one `shell:`
key, which is strictly better than an exemption marker in the same place — the
marker would be one line that leaves the dialect unstated, the key is one line
that states it. A valve is worth minting where the remedy is unavailable, and
here it never is. A missing `shellcheck`
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

**The walk is this gate's rule and lives in shared code, and
§check-action-run-shell's extractor is still not reusable here.** The rule above
— the job partition, the `gh` detector with its command-position roster and
logical-line joining, the checkout arm's `uses:` recognition with its ordering
line, the workflow/job/step `env:` ladder, and the valve's indentation binding —
is owned by this section and *held* in `native/src/actions.rs`, beside `walk.rs`,
because §check-action-permissions arrived as the second consumer that wanted the
walk whole. Only the **valve marker's spelling** is the caller's, handed to the
walk as an argument: this gate passes `gh-repo-exempt`, its sibling passes its
own token, and the required-reason rule and the bare-marker failure class are the
walk's and are shared. Duplicating the state machine to rename one string is the
parallel copy the content-tiering rule names as the defect, and the move was held
to byte-identical verdicts on the live tree and both fixture cases rather than
reasoned safe.

§check-action-run-shell's extractor is **not** that walk's third consumer, and
the standing rule it lives under still holds **between those two**. That
extractor carries a job partition of its own, because its dialect resolution
happens at the job boundary; what separates the two is therefore not a theory of
`jobs:`. What it emits carries none of the `uses:` or `env:` lines the
arms read; it must reassemble a block scalar byte-exactly and refuse five
constructs loudly, where this walk over-detects by design, carries no body
reassembly and refuses nothing.
What the two share is an *indentation convention* — find the job column, find the
job-key column — not a mechanism, and a helper
holding fifteen lines of convention for two differently-shaped state machines has
no invariant to hold. That the two now answer the same structural question is
exactly why the refusal is stated rather than assumed. Stated because the opposite reading is the natural one, and
because §lib/declaration.sh met the *same* standing rule with the opposite answer
— only the difference in what is being extracted decides it.

Tier `precommit`; the `# graph:` couples the surfaces §check-action-pinning
couples, `dir=one` — a one-way audit. A tree holding no YAML, or no job invoking
`gh`, exits clean on a zero count, the counted inertness that makes this kit
mechanism rather than a consumer gate. The `bad/` fixture opens on the attested
miss itself — the `v0.17.0` release job as it shipped — and carries further
reject jobs for the arms that one does not exercise, so §When a gate earns its
place's demand that a higher-false-positive
gate wait for a real miss is met by a fixture that *is* the miss rather than an
invention of one.

**Its implementation is a compiled subcommand**, on §check-action-pinning's terms
and for the same reason the two form one cohort: the declaration path is
`check-action-gh-repo.gate`, the rule runs out of the gate binary, and it was
proved byte-identical to the deleted shell gate. One consequence is worth naming
because the deleted file was where it was recorded: the shell implementation
required GNU awk's 3-argument `match()`, and the compiled one requires no awk at
all. The residual `gawk` floor was §check-gate-assertions' and, on its own
declaration's word, §check-action-run-shell's — never this gate's. **Both of
those halves are now gone, and the second was measured rather than inherited**:
the eighth budget batch ported §check-gate-assertions, and probing
§check-action-run-shell under a `gawk --posix` shim ran it **clean**, so its
`Requires GNU awk (3-arg match)` header was stale — deleted at that iteration's
close — and it holds no gawk extension at
all. That emptied the residue **this file named**, not the floor:
§check-docs-render-fidelity held it after them, and §check-gate-assertions records
the probe's scope. Its port under `shell-gate-tail-port` emptied the floor's live
set outright. What is *not* changed on that finding is the published requirement:
`awk (GNU)` is an element of `context-kit/lib/toolfloor.sh`'s probe roster held to
docs/install.md §Requirements by `check-install-toolchain`, and narrowing a
user-facing requirement is filed rather than taken in passing
(§check-gate-assertions).

### check-action-permissions

Invariant: in every Actions-shaped YAML file, a **job** that consumes the GitHub
token has the scopes it takes **declared** rather than inherited from an
invisible repository default. A `permissions:` block is an **allowlist**, so an
undeclared scope makes the read come back as an HTTP 404 — and a 404 on a read is
indistinguishable from an absent resource, which is why the failure arrives
looking like "no such Release" rather than "not permitted".

The class is concrete and this repo supplied it: site-kit's release-body arm
needs `contents: read` and would have reported an absent Release without it. On a
**public** repository the omission stays invisible, because the content is
anonymously readable anyway; it surfaces the first time a private-repo consumer
copies the workflow, which is exactly the shape a kit ships into.

Kit mechanism rather than a consumer gate, on §check-action-run-shell's and
§check-action-gh-repo's deciding fact reached by the same route: *kits* ship
workflow templates their consumers vendor — §templates/gates-workflow.yml and
site-kit's `templates/site-health.yml` — so a consumer gate could repair one tree
and leave every downstream vendor of the template unchecked. Named `action-`, not
`check-workflow-*`, for §check-action-pinning's reason, and that collision is not
hypothetical: `check-workflow-tiering` already governs §The workflow directory.

**Scan set, split predicate and walk are §check-action-gh-repo's, consumed
whole** — the derived `*.yml` / `*.yaml` walk from the scan root with the shared
prune set, the `jobs:`-key subject with a `runs:`-shaped composite action
**skipped and counted**, and the job-partitioned walk in `native/src/actions.rs`
this gate is the second consumer of. A composite action has no job and no
`permissions:` of its own, inheriting the calling job's, so the assertion belongs
to the caller. Nothing of that mechanism is restated here; what this section adds
is what the walk emits for it and what is done with it.

**The trigger — a job is armed by any one of three**, and the set is the same
detector the walk already runs for its sibling:

1. a step whose `uses:` ref before the `@` is `actions/checkout`;
2. a `gh` invocation in one of the job's `run:` bodies;
3. a `secrets.GITHUB_TOKEN` or `github.token` reference in the job's extent.

Trigger 3 is a **textual** match, and its over-detection is the safe direction —
§check-action-gh-repo's stated bias, inherited with the walk. A job that mentions
the token in a comment arms the gate and its author writes one `permissions:`
block or one valve; a job that consumes it silently is the failure this gate
exists for. `secrets.NPM_TOKEN` and every other secret are **not** matched: they
are not the GitHub token and carry no `permissions:` scope. `publish.yml`'s `npm`
job is the in-tree case that proves the distinction matters — it references
`secrets.NPM_TOKEN` and nothing else, and is correctly inert. The reference's
**line number is deliberately not carried**: a finding names the job, never the
evidence line, so the event is a bare marker that the job is armed rather than a
field populated at one transition and read at none.

**The scopes in scope for a job** are its own block where it has one, and the
file's workflow-level block otherwise. A job-level block **replaces** the
workflow-level one rather than adding to it — GitHub's own rule — so a job
declaring `id-token: write` alone inherits no `contents:` from a workflow-level
block that names it. **Modelling the inheritance is load-bearing, not
completeness**: site-kit's `templates/site-health.yml` and its filled instance
both leave the `probe` job's block at workflow level, so a gate implementing only
the job-level lookup would red a correct live file on day one and read as a false
positive — the shape §check-action-gh-repo's step-level `GH_REPO` lookup has, and
named here for the same reason.

**Scopes are read as key names with their values, and three value shapes are
recognised**, because GitHub admits all three and a gate reading only the mapping
form would answer wrongly on the other two: a mapping of
`<scope>: read|write|none` (block or flow); the scalar shorthands `read-all` and
`write-all`, which grant every scope at that level; and the **empty** mapping —
`{}`, or a `permissions:` key with an empty subtree — which grants nothing. The
two shorthands are not told apart, because nothing reads the difference: both
satisfy both arms below, and a distinction with no reader is a field populated at
one transition and read at none.

**Two arms, disjoined by what the trigger proves:**

- **Arm A — `contents:`, where the consumption is exact.** A job armed by trigger
  1 must have `contents:` at `read` or `write` in scope (`write` satisfies
  `read`; either shorthand satisfies both). A checkout fetches repository
  contents, so the scope is not a guess and the gate may name it.
- **Arm B — a declaration, where the scope depends on the call.** A job armed by
  triggers 2 or 3 and not by 1 must have a **non-empty** set of scopes in scope.
  Which scope a given `gh` call consumes depends on the subcommand, and a
  verb-to-scope map is a **vocabulary** — unbounded, provider-versioned, and
  the kind of rule content a kit must not ship as a literal
  (CLAUDE.md §The provenance seam). So this arm asserts that the job *says*
  what it takes, which is the whole difference between a reviewed allowlist and
  a repository default nobody in the tree can see.

A job armed by 1 takes arm A, which is the stronger; the arms are not mixed
within a job. A job armed by none is **inert and counted**, the counted inertness
that makes this kit mechanism: a consumer running no GitHub Actions, or none that
touches the token, pays nothing for it.

**`permissions: {}` fails both arms, and that is the gate working rather than a
harshness to soften.** An empty allowlist grants nothing, so a job that checks
out under it has no `contents:` and fails at runtime — on a private repository
immediately, and on a public one only once the content stops being anonymously
readable. That silence is the whole observation this gate is built on, and the
`bad/` fixture opens on it.

**A reusable-workflow call (`jobs.<id>.uses:`) has no `steps:`, so triggers 1 and
2 cannot fire in it.** Trigger 3 still can, and where it does the assertion is
right rather than incidental: a job handing the token to a called workflow passes
its own `permissions:` along with it, so declaring them is exactly the act the
arm asks for. The called workflow's *own* jobs are held by this gate in the file
that defines them — the same boundary §check-action-run-shell draws for a called
workflow's `run:` bodies. Stated because the silence would otherwise read as an
oversight.

**Output.** Clean is one line naming what was checked — armed jobs, inert jobs,
Actions-shaped files, walked files and the files the predicate skipped. A finding
names the file, the job id, its line and which arm failed; the two arms are two
failure classes and take **two `help:` lines** (§Output contract), one naming
`contents: read` and one naming the declaration. The bare-marker class inherited
with the walk keeps its own third.

**The valve.** `# action-permissions-exempt: <reason>`, the kit's established
marker shape, reason **required and non-empty**, binding by indentation exactly
as its sibling's does (§check-action-gh-repo owns the binding rule): at or left
of the job-id column it precedes a job, at the step-list dash column it precedes
a step, inside a step's keys it binds that step. A job-bound marker skips the
job; a step-bound one drops that step's evidence from the trigger set and leaves
the job's other evidence held. A valve is minted here — unlike in
§check-action-run-shell, where the remedy is always one `shell:` line — because
the remedy is **not** always available: the `gh` detector over-detects by design,
and a job reaching the token through a third-party action's own machinery is
outside this gate's theory.

**The refusals, and the fidelity limits that ship with them.** Exit 2, naming the
construct, and only inside the Actions-shape subject: a `permissions:` value that
is neither a mapping, nor `read-all`/`write-all`, nor empty — a `${{ }}`
interpolation, an anchor or an alias. The gate cannot resolve what such a block
grants and does not guess.

*Out of ability, stated as cost rather than papered over.* A **third-party action
that consumes the token implicitly** — passed through its own `with:` inputs or
read from the environment — is not detected; the trigger set is checkout, `gh`
and an explicit token reference, and widening it by guessing at action names
would be a maintained roster of someone else's software. A **`curl` to
`api.github.com`** is likewise undetected: it is a bare URL in a shell body with
no token binding the gate can see. Both take the valve, and both are the honest
limit of a tree-local reader.

*Not asserted, deliberately.* That a declared scope is **not wider than
necessary**. Over-declaration is a real hazard and it is a different gate: it
needs the verb-to-scope map arm B refuses to ship, and asserting a minimum is
mechanically decidable where asserting a maximum is not. Named so a later reader
does not read the silence as an omission.

Tier `precommit`; the `# graph:` couples the surfaces §check-action-pinning
couples, `dir=one valve=none` — a one-way audit with no leading surface, the
`valve=` field being §The `# graph:` manifest's cycle-valve classification and
carrying no relation to the exemption marker above. **No new knob**: the scan set
is derived, the prune set is the shared one, and the asserted scope name is
GitHub's own published vocabulary rather than rule content a consumer could own
differently — a knob here would be a scope-roster surface with no second consumer
and nothing private to externalize.

**Its implementation is a compiled subcommand, born native.** The declaration
path is `check-action-permissions.gate` and the rule runs out of the gate binary
(§Porting a gate to the binary substrate — new gates here are born on that
substrate, so there is no shell form to prove byte-identical against). The
verdict-invariance obligation this gate did carry is its *sibling's*: widening the
shared walk's event stream for it had to leave §check-action-gh-repo's findings,
counts, clean line and exit code byte-identical, which was proved by driving both
binaries from the same cwd with the same argv over the live tree and both fixture
cases rather than reasoned.

### check-commit-msg

`checks/check-commit-msg.gate` (`commit-msg`, binary-dispatched).
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
synthetic session UUIDs the tracked `smoke/` trees legitimately carry; the
account-identification class ruled below is the fourth). A
no-argument run (the whole-tree battery)
is a clean skip: the message is not a tracked surface and the history-scan
backstop is deferred to the hosted-attestation rung. A missing message-file
argument-with-value, or a missing required tracked pattern file, is fail-closed
(exit 2). The `# graph:` couples the pattern file (the regeneration trigger),
not a tree path — the gate is emitted into the commit-msg hook, not the
pre-commit hook. Subject *shape* is the sibling check-commit-subject's job:
this gate stays the leak guard, that one the parse guarantee.

**The account-identification class bans a shape, not a name.** Public prose
cites the *role* — "the account carrying `workflow` scope" — and never the
account filling it, because forge accounts are volatile and naming one
correlates identities. A denylist of names cannot carry that rule: it catches
only a name someone already thought to list, and the attested leak was a handle
nobody had listed. So the shipped pattern matches the shape — a **handle-shaped**
backticked token within a few characters of an account noun, in either order —
and spells no handle, domain or project term of its own, which is what lets it
ship tracked rather than local (CLAUDE.md §The provenance seam) and hold in a
fresh clone carrying no private list. Handle-shaped **excludes a short
all-lowercase token**, which is a CLI name — `gh`, `git`, `npm`, `cargo`, `ssh`
— rather than an account.

**That exclusion is what makes it tree-exact, and tree-exact is a stricter
calibration than a message-only guard would take.** One pattern source feeds two
readers (§check-tree-terms) whose economics do not match. An over-refusal in a
*message* is argued down once, by a human, at the one commit it blocks — a cost
worth paying for a guard that must not under-refuse. An over-refusal in the
*tree* is a standing red clearable only by rewriting prose that was never wrong,
and rewriting a tracked sentence to satisfy a heuristic inverts the rule the
heuristic serves — flatly so where that sentence is a quotation, which editing
would falsify. So where the two tolerances disagree the tree's wins.

**Its honest limit, stated rather than discovered:** the scope is handle
identification and nothing wider. Account *topology* — that one machine holds
two accounts, that one of them maintains this repo — is a proposition rather
than a token, and no pattern reaches a proposition. A message can leak the
topology while matching nothing here. That residue is a review obligation, and
naming it here is what keeps a green gate from being read as the whole
assurance.

**The pattern set crosses the bridge as two arrays, `GATE_MSG_PATTERN_FILES` and
`GATE_MSG_PATTERN_FILES_LOCAL`**, resolved in `lib/gate.sh` from the two
consumer scalars by the same unquoted expansion the resolver itself used, so
word-splitting and pathname expansion keep the semantics they had.
`gate_msg_pattern_files` reads those arrays rather than re-expanding the scalars,
which keeps one derivation for both its readers — this gate and
§check-tree-terms — and gives the distinct spelling §lib/gate.sh's naming rule
grants a scalar that feeds an array.

**Matching runs through the crate's own POSIX ERE engine** (§The POSIX ERE
matcher) rather than through `grep -E`, which subtracts nothing: the engine was
built for this grammar and the shipped pattern shapes are unit-tested against it
beside the pair.

**One adjudicated disagreement, on a fail-closed path, and the compiled side is
the better one.** Given a malformed pattern both substrates exit 2 and both emit
the same `fail_closed` line; what differs is the diagnostic above it. The shell
fed its pattern set to `grep` through a process substitution, so `grep` named the
ephemeral `/dev/fd/N` path and a line number inside it — a location the reader
cannot open. The compiled form names the offending **pattern** and what is wrong
with it. Recorded rather than smoothed over, on the rule that a disagreement is a
finding adjudicated against the rule (§The port-candidate criteria): the rule
here is the §Fail-closed contract, which the two satisfy identically.

### check-commit-subject

Invariant: the prospective commit message's subject line (the `commit-msg`
hook's `$1`, first line) parses as `<type>(<scope>)?!?: <summary>` with
`<type>` drawn from the shared roster and `<scope>` a `[a-z0-9./-]+` token, or
matches a git-generated carve-out — `Merge `, `Revert ` and the `fixup! ` /
`squash! ` autosquash forms. A subject that does not parse is an unread write
to a governed projection, not a style nit: the trajectory arm's feat/debt column
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
roster's single home is lib/gate.sh; drift-kit's kpi-task-split and the
trajectory arm keep their own class mapping (feat vs fix+refactor), a
classification over roster tokens rather than a second roster. Edge behavior
matches check-commit-msg: a no-argument run (the whole-tree battery) is a clean
skip — the message is not a tracked surface; a missing message-file
argument-with-value is fail-closed (exit 2). The `# graph:` couples the
roster's config home (lib/gate.sh, the regeneration trigger), not a tree path —
the gate is emitted into the commit-msg hook.

### check-tree-terms

`checks/check-tree-terms.gate` (`precommit`, binary-dispatched).
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

**The port owes no bridge work on the pattern-file half, and the two halves read
one resolution rather than two copies of it.** `GATE_MSG_PATTERN_FILES` and
`GATE_MSG_PATTERN_FILES_LOCAL` already crossed the config bridge for
check-commit-msg, and this member declares the same two knobs and reads the same
resolved arrays — through the same two helpers, which the sibling module exports
rather than this one re-deriving. `gate_msg_pattern_files` is one function in the
shell precisely so the two halves of the leak ban cannot drift apart, and a
second compiled copy of that resolution would reinstate the drift the shared
helper exists to prevent. **No pattern is baked into the crate**: the roster is
consumer config on the §check-graph pattern, and a kit literal carrying a
consumer's vocabulary publishes it (CLAUDE.md §The provenance seam).

**The cheap-filter-then-match split survives the port.** A fork-free per-path
filter runs first — prune dirs, the `msg-patterns` prefix, the regular-file test
— and the pattern set is compiled **once for the whole walk** rather than per
file. A port recompiling per file would be the regression the split exists to
prevent, and it is stated here because a compiled substrate makes the wrong shape
cost nothing visible.

**Three behaviors the shell form held by accident of `grep` are ruled, not
inherited.**

- **A GNU escape in a consumer's pattern file is refused, loudly.** The crate's
  POSIX ERE engine (§The POSIX ERE matcher) refuses `\b`, `\s`, `\w` and the rest
  of the GNU set **by name** at compile, which this member turns into exit 2 — a
  pattern the engine cannot honour must never read as a tree with nothing in it.
  The divergence is not this member's to invent: it already existed on the same
  pattern set through the ported check-commit-msg. What changes is that it is
  stated, so a consumer meeting the exit-2 has its cause. The POSIX ERE spelling
  of a word boundary is `(^|[^A-Za-z])term([^A-Za-z]|$)`, and docs/ddd.md's
  worked adoption example is written in it for that reason.
- **A banned shape inside a tracked binary yields a path-only record, and reds.**
  The compiled form detects the NUL byte, matches on the decoded text and emits
  the path **alone** — no `:lineno:line` suffix — rather than dumping lossily
  decoded bytes: a binary's bytes can carry no newline for a megabyte and can
  carry control characters, either of which corrupts the `path:lineno:line`
  grammar the red output rests on, and the record's purpose — naming a line to
  edit — does not exist for a binary anyway. **This is a hole closed, not a
  format choice**, and the measurement is why it is recorded rather than assumed:
  GNU grep from 3.5 writes `grep: <path>: binary file matches` to **stderr**, and
  the shell form captured stdout only, so a leak inside a tracked binary reported
  `TREE-TERMS: clean` and exited 0. Dead on this tree, which tracks no binaries;
  live in a consumer's, which is exactly why it is ruled here rather than
  discovered there. **It is live on this tree**, which the ruling did not expect
  and the measurement found: `installer_smoke`'s artifact legs commit the gate
  binary into a scratch consumer's tree at `scripts/checkwright-gates`, so this
  arm reads a real consumer's real artifact on every run of that suite. What it
  caught there is **fixed at its source rather than exempted here**: the build
  remaps the builder's paths out of the artifact and verifies the result against
  this same banned set (§build-native), so what ships carries no such path. The
  arm stands unnarrowed, which is the outcome worth recording — this guard's
  first live encounter removes a leak rather than earning an exemption.
- **Its fail-closed arms are discharged where they are held**, by
  §check-gate-fail-closed across the whole registry rather than banked as this
  member's own coverage, on §Fixture-pair discipline's terms. **What is not an
  arm, stated because it is a narrowing against the shell form:** an unreadable
  tracked file is *skipped* by the content match rather than failing it closed,
  so a path the walk selects and the read cannot open leaves the scan silently
  short. The shell form failed closed there.

**The authoring rule this member's own module carries, and it runs both ways.**
`native/src/gates/tree_terms.rs` is a tracked file **inside the corpus this gate
scans as content**, so a banned shape spelled literally in its source or its unit
tests reds the gate against itself; the module **composes** such a shape at
runtime instead. `native/src/gates/commit_msg.rs` already does this and names
this gate as the reason. The constraint runs the **opposite** way for everything
under `gate-tests/`, which is pruned and may spell banned shapes freely — and
does, because that is what makes the pair's skip cases prove a skip rather than
an absence. Getting the two directions backwards is the plausible error, which is
why both are stated. The module's own test inputs come from the **fixture**
pattern files, never from the consumer's tracked list: compiling a live pattern
into the crate would be a consumer-config literal no gate catches, since the
leaked string is not itself a banned term and this very gate cannot see it.

**Criterion 4 binds, and the price is paid: the pair was widened before the
port.** Its corpus is `git ls-files` over the whole tracked tree, pruned only by
the shared prune dirs and the `msg-patterns` basenames, so **every** registry
member's declaration path lies inside the corpus it scans as content — criterion
4's predicate verbatim, and reached through the **walk** rather than the trigger
field, which is `couples=scripts/msg-patterns.list` and selects no declaration at
all (§The port-candidate criteria, whose couple-clears-walk-binds register this
member joins). The pair stood at two of twelve control arms and was widened first
in three directions: `good/` gained a pruned-directory file, a `msg-patterns.list`
and a `msg-patterns.local.list` — the second proving the self-exemption is a
prefix glob and not an exact-name match — each carrying a banned shape so its skip
is proved by **greenness** rather than by absence; `good/patterns.list` gained a blank line and
the live shapes the fixture omitted, including the only anchored one, whose ERE
anchoring a port can silently drop; and `bad/` gained a second leaking file and
one carrying two leaks on a line plus two on identical separate lines, which pins
the record grammar, one record per matching *line* however many patterns hit it,
and no dedup.

**Two facts about the fixture corpus fixed that shape, and a bespoke
`check-tree-terms.test.sh` carries the residue.** A case dir is **not** its own
repository: `git ls-files` inside one returns the **outer** repo's index scoped to
that subdir, printed relative to cwd. So the non-repository arm is structurally
unreachable from a case dir and takes the unit-test arm; the `gate-tests` prune
never fires on a case's emitted paths, which is why the pair works at all and why
it is inert under the port; and a widening file that is written but not `git
add`ed is **invisible to the gate**, so an untracked case silently under-covers.
The test file takes what remains: the non-repository fail-close, the
missing-required-pattern-file fail-close and with it the entire env-knob
resolution path — which both cases short-circuit by passing a positional — the
empty-pattern-set *tree unchecked* clean, the binary path-only record, and the
record **order** a pair cannot assert, each `expect.txt` line being an independent
substring test.

**A fourth direction the pair once carried lives there too, and for a different
reason — it is an arm a case dir *may not ship*, not one it cannot reach.** The
widening's dangling symlink, whose own blob content is a banned shape so its skip
is proved by greenness, was a **tracked** symlink inside `good/tree/`, and a
fixture is payload content (§Consumer payload): `tar` cannot create a dangling
link on a native Windows host, so that one path aborted the vendor mid-kit. The
arm relocates rather than retires — the `.test.sh` ships too, so consumer-side
coverage is unchanged — and constructs the link at run time in its own sandbox
repository, `git add`ing it and asserting the index really holds mode `120000`,
since an unstaged plant leaves the arm running over nothing. **The link must stay
dangling**, which is the trap here: the module filters with `is_file()`, which
**follows** the link, so a resolvable symlink is *scanned* rather than skipped and
would both change `good/expect.txt`'s count and invert what the arm proves. The
arm **skips and declares** where `ln -s` fails, creating a symlink needing a
privilege an ordinary Windows account may not hold — the honest shape for an
assertion whose precondition is a platform capability, and the same capability
that made the tracked form unshippable. **No `expect.txt` moved:** `good/`'s
remaining paths still scan as `2 tracked file(s) scanned`, the symlink having been
excluded by `is_file()` all along, so this is a relocation and not a reduction.
Why no gate holds the invariant instead: §Fixture-pair discipline's stated
non-target.

**The parity run's verdict, recorded as the register §The port-candidate criteria
sets for a member binding criterion 4.** Assertion A forbids a descriptor and a
script coexisting in one resolve dir, so the cross-substrate comparison ran on
the **pre-descriptor** tree. Five comparisons — both fixture cases, the live tree
with no argument, a live subdirectory scan root, and the knob-resolved pattern
path — agreed on stdout, stderr and exit code in every one. **Parity is proved
over the pair**, the only corpus inert under the port; the three live-tree arms
are **no disagreement found on the pre-descriptor tree**, never parity proved.
The two ruled behaviors above are deliberate divergences and sit outside that
claim: the GNU-escape refusal and the binary path-only record are each a place
the compiled side is the better one, and each was probed on both substrates
rather than reasoned about.

**The verdict is priced, not held**, and this member declares no `# port-until:`
on that ground. Criterion 4 orders a port and prices it; it has never made a
member un-takeable, five members having bound it and ported by widening their
pairs, so a field whose meaning is *not takeable now* is the wrong instrument for
a fixture widening.

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
- `lib:` — `_`-bearing lowercase identifiers in **command position**: the leading
  edge of a line, or directly after `;`, `&`, `|`, `)`, `$(`, `&&` or `||`. The
  intent is the sourced-lib API, and the class is wider than that intent by
  construction — a locally declared function is matched by its own call sites like
  any other, and a `case`-arm pattern sitting at a line's leading edge in front of
  its `)` is matched too. Both are deliberate as of §The sixth budget batch, which
  **corrected this description against the code** rather than narrowing the code:
  a narrowing the shell form never made would be a verdict change across the port
  seam, and the sentence was the half that had drifted from its subject.
- `knob:` — uppercase `_`-bearing names read via the defaulted-env idiom
  `${NAME:-…}` / `${NAME:=…}`.

The `case:` class reads the arm's **action, never its pattern**, and that is a
**privacy boundary** rather than a parsing convenience: a consumer's arm patterns
are its own rule vocabulary, which a kit gate must never read (CLAUDE.md §The
provenance seam). The gate asserts that a consumer's divergent rule lines are
*declared*, never what they say. The discard is what enforces it, so the compiled
form reproduces it **as a discard** and never as a capture that is later ignored:
a port that parsed an arm into a pattern and a body and kept the pattern — for a
diagnostic, a debug rendering or a test fixture — would have put consumer
vocabulary inside the crate, and no committed case would catch it, because no case
inspects intermediate state. `func:` is inert on every pair in this tree (thin hook
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

**Ported to the binary substrate at §The sixth budget batch.** Two behaviours the
port **reproduces rather than repairs**, on the rule that a refusal the shell form
never made is a verdict change across the seam. The gate **refuses outright** —
exit 2, the fail-closed message — on a paired file carrying no
knob-with-default idiom at all, reporting *could not classify* where the honest
verdict is *no knobs*. It fails *closed*, which is
what makes reproducing it safe-but-wrong rather than dangerous, and the tree is
green only because the two knob-less files in its corpus are excluded by the
`*-config.sh` rule before the call. And the `lib:` class matches what the bullet
above now says it matches. The refusal is filed as its own unit —
`template-copy-parity-knobless-refusal` — rather than repaired here.

**Its root default is a derivation no injected case can reach**, since every
committed case passes an explicit root — so the discharge is a sibling harness,
`check-template-copy-parity.test.sh`, which drives the `git rev-parse
--show-toplevel` branch from a *subdirectory* (a cwd-relative default would find
no pair and pass vacuously) and drives its refusal with git unable to answer.
Criterion 4 **clears** on this member: its glob is exactly two levels deep and
cannot reach a `checks/` segment at all, taken off the walk rather than off an
earlier section's sentence — a property of a gate against a consumer's config, so
a consumer whose gates dir held a shell declaration matching a kit template
basename would flip it.

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
`<kit>/templates/<name>.list` has a sibling **directory** `<kit>/<name>/` holding
the artifacts the list registers, **or** when the binary declares that it
natively dispatches that registry; a `.list` with neither is skipped-and-counted,
not failed, the same silent-skip exclusion its sibling gate gives an unpaired
template. Shipped members are the basenames of the sibling directory's `*.sh`
files, extension stripped, read from `git ls-files` so an untracked scratch file
forces no registry line, **joined with** the natively dispatched member names;
registry members are the non-comment, non-blank lines — the `gates.list` grammar
`gates_list_members` reads, the same grammar the consumer's own KPI resolver
reads its registry through, so the gate calls a name registered exactly when that
resolver would.

**Native dispatch is a shipping mechanism, and the widening says so rather than
working around it.** A kit whose registry members moved into the binary ships
them as surely as it shipped files — the members answer the same registry names
at the same transition — so the gate's question is *what answers this name*, not
*what file answers it*. The gate **predates the substrate**, and every later kit
port meets it; enforcement-first puts the fix in the unit that first breaks it
rather than in a shim, which is why thirteen files whose only purpose was to
satisfy this gate were refused: they would each then need their own port
disposition, converting a blind spot into permanent corpus. Scoping the gate off
the kit was refused as scoping enforcement off the thing it exists to check.

**The widened predicate is not satisfiable by the population going empty, and
that is its load-bearing constraint.** Emptying a sibling directory reds every
registry line, while *deleting* it takes the template out of the sibling arm
entirely — and a narrowing that removes a violation by removing the check is
strictly worse than the red, because a later session reads a green board with no
way to see it. The native declaration is what holds such a template in
population, so a declaration whose roster is empty reds every registry line
through assertion (B) rather than skipping. **Assertion (C)**
closes the declaration's own side: where the declaring kit is in the scan's root
set, its registry template must have been reached, so a declaration outliving its
template is a finding. Scoping (C) to the enumerated roots is what keeps a
sandboxed fixture — which vendors neither the kit nor its template — out of it.

**The honest limit.** The population predicate is *inferred*, from a sibling
directory or a native declaration, and there is no third signal. A tree that
deletes a registry's artifacts **and** its native declaration in one motion
degrades that template to skipped, silently. Nothing here can catch that, because
deleting the declaration is deleting the dispatch, and the gate cannot tell an
intentional retirement from an accidental one. It is named rather than closed.

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
a template* for a consumer to adapt. It is a template rather than a bundled
member, so it is not a shipped artifact and is not required in the registry — the
shipped set is the sibling directory's contents joined with the natively
dispatched roster, and nothing else.

Sweep: kit roots come from `gate_kit_roots` (the `GATE_SDK_KIT_DIRS` knob —
§Layout and configuration), the sibling roster meta-gates' shape; config adds
**no new knob**. Positional form `check-template-registry-parity [root]`
resolves relative kit roots against a fixture tree (the case dir's
`gate-sdk-config.sh` names the fixture kits); bare, it sweeps against the git
toplevel. Fail-closed: a non-repo cwd with no root argument, an empty kit
roster, an unreadable template or sibling directory, or a failed `git ls-files`
is exit 2, never a false clean. `dir=bi` — either side going stale is the
defect, which is what (A) and (B) split between them — tier `precommit`. The
fixture pair synthesizes both sides at once: `good/` proves green on a registry
in parity while carrying both structural exclusions (a `.list` with no sibling,
and a non-`.list` template), and `bad/` proves a registry that is one-sided in
each direction at the same time. The pair also carries the widening: `good/`
holds a kit whose registry has **no sibling directory at all** and is in
population anyway, on the native declaration the binary carries, and `bad/` holds
that same shape one-sided in both directions — the case the sibling-only
predicate cannot reach at all.

**Its implementation is a compiled subcommand**, on §check-action-pinning's
terms — declaration path `check-template-registry-parity.gate`, rule out of the
gate binary, proved parity-identical before the shell gate was deleted
(§The kit-roots `gate_kit_roots` cohort), which records the criterion-4 finding
this member carries. The finding changed nothing above.

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
