# SPEC amendment: knob-files-cut-3

Queue entry: `config-seam-static-format`, its third increment. It leads unit set
`config-seam-third-cut`: **cut 3 is canon-kit, context-kit, delegation-kit and drift-kit**, and it
rules the command-knob and knob-reference shapes (operator direction, 2026-09-14, lead-relayed).
The format, the precedence order, the legacy refusals, the bridged-input closure, the
`--emit knob-values` arm and the validator hook are cuts 1 and 2's and stand (gate-sdk/SPEC.md §The
knob file). This amendment adds only what the four kits need beyond them. It lands after
SPEC-knob-file-couples.md, so none of the four kits' readers takes a hand couple.

Scope's promotion recorded three items for spec to settle, and each is ruled here. The command knob
is delta 2 and the knob reference delta 3. Retiring canon-kit's `GATE_SDK_RESOLVING_KNOB` gating is
delta 2 as well, and delta 11 keeps the variable for the transitional shape. The third item was
whether the cut removes `check-prose-enum`'s time above `check-core-files`. Probed at `c8059acc`, it
does not. `--only check-prose-enum` takes 768–788 ms and `--only check-core-files` 106–111 ms. The
member's bridged resolution (`gate_knob_env check-prose-enum`) takes 121–123 ms against 41–42 ms, and
62–63 ms of the difference is the `--emit enum-sets` spawn, which the crate still pays after this cut.
The gate's own execution takes the remaining ~630 ms. So the cut removes at most ~80 ms there, and
the nested bridge the spawned front-end sources stays until gate-sdk's cut.

Authoring found six further items the probe did not reach, and each is ruled here: drift-kit's KPI
plugin contract exports consumer-declared knobs (delta 4); five defaults read the gates directory,
two probe the filesystem, and three read the harness's home (delta 5); two set-ness refusals and one
retired-name refusal (delta 6); two post-config compositions (delta 7); and a drift-kit template that
sources canon-kit's shell config (delta 8).

## The seam

- **Kit mechanism:** the four defaults tables and three validators under `native/src/knobs/`, the
  command-knob spawn and its output adapters, the reference form in `native/src/knobfile.rs` and
  `native/src/knobs/mod.rs`, the open scalar family, the placeholder origin, the retired-name table,
  the reader-side compositions, the four comment-only `.knobs` templates, and the rewritten
  `kpi-deprecated-surface.sh` template.
- **Consumer config:** each `<gates-dir>/{canon,context,delegation,drift}-config.knobs` and its
  gitignored `.local.knobs` overlay. They stay the adopter's edit seam, so gate-sdk/SPEC.md §The
  config-seam port disposition's ground holds, and only the substrate changes.
- **This repo's own:** the four `scripts/*-config.knobs`. canon's carries this repo's ruling-record
  markers, abbreviation allowlist, claim commands and surfaces; context's its budgeted sections and
  ratchet paths; delegation's its read-only type roster and meta paths; drift's the stage-roster
  reference and icebox section. All are consumer values and never become kit literals.
- **Private rule content:** none newly in reach. The vocabularies canon-kit's commands emit stay in
  this repo's `scripts/*.sh` emitters, which are consumer files the kits never read as literals.

## What changes

### (1) The four kits go static {mechanical}

canon-kit, context-kit, delegation-kit and drift-kit gain defaults tables,
`native/src/knobs/{canon_kit,context_kit,delegation_kit,drift_kit}.rs`, and join `STATIC_KITS`.
Every knob, shape and default transcribes from its library as it stands at this amendment's commit,
with the exceptions deltas 2 to 7 rule. The four libraries stop resolving anything and are deleted
by delta 9.

Every live value in the four consumer configs is expressible in the line grammar once deltas 2 and
3 land, and none needs quoting. Two cases look hard and are not:

- **A pathspec with magic**, `CONTEXT_KIT_GROWTH_PATHS[] = :(exclude)docs/`: the value is verbatim to
  end of line, and `(`, `)` and `:` are data.
- **An ERE**, `CANON_KIT_INSTALL_SECTION_RE = ^(Quick start|Install)`: verbatim, with no escape.

**The session-context hook's names get no row.** `CONTEXT_KIT_DRIFT_REPORT`,
`CONTEXT_KIT_STAGE_RULES` and `CONTEXT_KIT_SESSION_ROLE_FILE` are read by the consumer's copy of
`templates/session-context.sh` from its own environment, and they never crossed the bridge. The hook
also reads `CONTEXT_KIT_STATE_FILE`, `CONTEXT_KIT_MEMORY_DIRS` and `CONTEXT_KIT_ENV_PROFILE_FILE`
that way, beside their rows. That read is unchanged, which is cut 2's `LIFECYCLE_KIT_SESSION_ID`
precedent.

**`DRIFT_KIT_CONFIG_FILE` leaves two registry declarations.** The library reassigned it to the
resolved config path, and `--emit-trajectory` and `check-trajectory-fresh` declare it, but no
compiled code reads it (`native/src/emit/mod.rs`, `native/src/gates/mod.rs`). The static loader's
set-but-missing refusal on `DRIFT_KIT_KNOB_FILE` is the behaviour the declaration bought, so the two
declarations are deleted rather than renamed.

### (2) The command knob {design-bearing}

gate-sdk/SPEC.md §The knob file's command-knob bullet becomes a ruled shape. **Not yet applied:**

> **A command knob** (`<KIT>_…_CMD`) is an **indexed** knob holding an argv, one element per line:
> `CANON_KIT_MEASURED_CLAIMS_CMD[] = bash` then `CANON_KIT_MEASURED_CLAIMS_CMD[] =
> scripts/measured-claims.sh`. Its reader spawns the argv directly and no shell parses it, so a
> consumer on native Windows names its own interpreter. The program is resolved by the crate's one
> program resolver (§Fail-closed contract), the child inherits the reader's working directory and
> environment, and an empty knob means *no command*, which each reader already takes as its skip. A
> reader appending an operand appends it after the argv's last element. What a failed spawn or a
> non-zero exit means stays each reader's own contract.
>
> A command knob takes no environment override, because it is indexed. A command that is itself a
> front-end arm must not declare the knob that names it, or the resolution recurses.

Eight knobs take the shape. The five canon-kit knobs lose their bridged output pairs, and each of
the other three keeps its reader's contract except for the spawn:

- **`CANON_KIT_ENUM_SETS_CMD`, `CANON_KIT_INSTALL_TRANSPORTS_CMD`, `CANON_KIT_PAYLOAD_CLAIMS_CMD`,
  `CANON_KIT_MEASURED_CLAIMS_CMD`, `CANON_KIT_CLAIM_CLASSES_CMD`.** Each member reads its command's
  output through an adapter in `native/src/spec.rs`, which spawns the argv once per process and
  caches the parsed lines. The validation `lib/spec.sh` performed moves with it. For the enum sets,
  a command error, a line with no tab, an empty field or an extra tab is exit 2. For the three
  `<id>`⇥`<ERE>` vocabularies and the measured claims, so is an id that is not slug-shaped or a
  repeated id, with the knob named as the label. `spec::claim_vocabulary` takes the command knob in
  place of the two array knobs.
  **The ten output arrays retire:** `CANON_KIT_ENUM_SET_NAMES` / `_MEMBERS`,
  `CANON_KIT_MEASURED_KEYS` / `_VALUES`, `CANON_KIT_INSTALL_TRANSPORT_IDS` / `_PATTERNS`,
  `CANON_KIT_PAYLOAD_CLAIM_IDS` / `_PATTERNS` and `CANON_KIT_CLAIM_CLASS_IDS` / `_PATTERNS`. They
  existed only to carry output across the bridge's tab-joined wire, canon-kit/SPEC.md already says
  none is consumer-authored, and a table row for one would be a knob a file could set to nonsense.
  So is the `GATE_SDK_RESOLVING_KNOB` gating that computed them, because an in-process adapter
  spawns only for the member that asks.
- **`CONTEXT_KIT_HOOK_CMD`.** `--emit-always-loaded` spawns the argv where it ran `bash -c`, and a
  spawn failure still reads as `hook 0`. `--emit-drift-report`'s built-in `kpi-always-loaded`
  reads the same value.
- **`DELEGATION_KIT_REFRESH_CMD`.** `--usage-verdict` spawns the argv where it ran `bash -c`.
- **`DELEGATION_KIT_LIVENESS_CMD`.** The turn-end hook spawns the argv with the scratch dir appended.
  Its resolution predicate stays *executability*: the override resolves to no reader, and every
  firing reads `unavailable`, when the argv's first element resolves to no executable program. A
  one-element override naming an executable path behaves exactly as before. The *no interpreter
  word* clause goes, because it existed so the kit would never choose an interpreter, and an argv
  lets the consumer choose one.

**Refused, with grounds (they stay here and in git history).** Keeping the ten arrays as derived rows
that spawn the command. It keeps names whose only purpose was the bridge, and index alignment
between two rows would rest on a per-process spawn cache that one adapter holds by construction.
Keeping `CONTEXT_KIT_HOOK_CMD` and `DELEGATION_KIT_REFRESH_CMD` as shell strings, which cut 2 did for
`QUEUE_KIT_LESSON_SINKS`. That knob is not `_CMD`-named, and its arm's contract *is* `bash -c`, while
these two are named command knobs, and the rule's native-Windows ground reaches them whole. A
per-knob spelling choice, scalar string or argv. It gives one name two grammars, the defect
§lib/gate.sh's prefix rule exists against.

### (3) The knob reference {design-bearing}

gate-sdk/SPEC.md §The knob file, the grammar block and the paragraph on what the grammar lacks.
**Not yet applied:**

> A fourth line form splices another knob's value:
>
> ```
> NAME[] <- OTHER         the resolved elements of OTHER, at this position
> ```
>
> A reference line carries no `=`, so no line in the three value forms can read as one. `NAME` and
> `OTHER` are SCREAMING_SNAKE names and blanks around `<-` are trimmed. The line contributes OTHER's
> **resolved** value, whatever layer supplied it, as elements of `NAME` in file order among `NAME`'s
> own element lines. `NAME` is then replaced whole by its lines in that file, exactly as without a
> reference.
>
> A reference is refused with its file and line, exit 2, when:
>
> - `NAME` or `OTHER` is not a declared **indexed** knob. The form splices elements, and no scalar or
>   keyed reference has a reader.
> - `OTHER` is not statically owned. A bridged value is not the crate's to resolve.
> - `OTHER`'s row reaches a bridged input (§the derived-default paragraph). The member's `--knobs`
>   closure is computed from declarations and cannot see a file's reference, so such a referent could
>   ask for an input the bridge never carried.
> - `OTHER`'s own resolved value carries a reference, or `OTHER` is `NAME`. Resolution is **one
>   pass**, the bound the `couples=` `knob:` token takes, so no cycle detector is needed.
>
> A reference may cross kits. It is the one splice form the file admits, and the grammar still has no
> expansion inside a value: `$`, `<-` and `${…}` in a value are data.

The two live instances:

- **Same-kit.** `scripts/canon-config.knobs`: `CANON_KIT_MEASURED_SURFACE_GLOBS[] <-
  CANON_KIT_MANIFEST_FILES`, then `[] = .claude/commands/*.md` and `[] = TASK-QUEUE.md`.
- **Cross-kit.** `scripts/drift-config.knobs`: `DRIFT_KIT_STAGES[] <- LIFECYCLE_KIT_STAGES`. It
  replaces the `gate_static_knob` call in `scripts/drift-config.sh`, which is deleted, and
  drift-kit/SPEC.md's `DRIFT_KIT_STAGES` bullet teaches the reference in place of the helper.

**A reference reaches a trigger.** SPEC-knob-file-couples.md's derivation gains one rule. A kit a
member's declaration reaches also reaches every static kit a reference line in that kit's **tracked**
knob file names as a referent's owner, read from the file at derivation time. That is how
`check-trajectory-fresh` gains `scripts/lifecycle-config.knobs`. The local overlay is not read, for
the reason the derivation already gives. A reference added or removed in a tracked file changes the
generated hook, and `check-graph` assertion D reds until it is regenerated.

Mechanism: `knobfile::Form` gains `Reference(String)`. `knobs::layer` stores an indexed knob's lines
as a sequence of elements and references, and `lookup` materializes it, refusing per the list above.
The unit tests cover each refusal and the two splice positions.

**Refused.** A value-side sigil, `NAME[] = ${OTHER}` or `@OTHER`. It breaks *the value is verbatim*,
and a glob or ERE may legitimately carry either spelling. A kit-side derived default
(`CANON_KIT_MEASURED_SURFACE_GLOBS` defaulting to the manifest globs) changes a shipped default for
every adopter to serve this repo's composition. A transitive reference buys nothing the two instances
need and costs a cycle detector.

### (4) drift-kit's knob family stays open to consumer scalars {design-bearing}

drift-kit/SPEC.md §The KPI plugin contract exports every scalar `DRIFT_KIT_*` value to a child plugin,
*a knob a consumer's own `drift-config.sh` declares and nothing in this repo names* included, and
`drift-kit/smoke/install.sh` asserts one. A static table refuses an undeclared name, so the contract
would break. gate-sdk/SPEC.md §The knob file, after the refusal list. **Not yet applied:**

> **A kit may declare its family open to consumer scalars.** In such a kit's files, a scalar line
> naming an undeclared name under the kit's prefix is a consumer knob, not a refusal. It resolves
> through the same precedence as a declared scalar, it has no default and no validator, and the one
> reader of the set is the kit's family read (`knobs::family`). An undeclared name written as an
> indexed or keyed line is still refused, and so are the kit's own locator names, `<KIT>_KNOB_FILE`
> and `<KIT>_CONFIG_FILE`, and any retired name, since a file setting one of those is a mistake the
> family would otherwise hide. drift-kit is the one open kit, for its plugin contract.
> The honest limit: a misspelled declared name in an open kit's file reads as a consumer knob rather
> than a refusal.

`--emit-drift-report`'s `child_env` reads `knobs::family("DRIFT_KIT_")`: every declared scalar's
resolved value, then every consumer scalar in the local overlay or the tracked file. A consumer scalar
exported in the invoking environment reaches the plugin by inheritance. drift-kit/SPEC.md's
*honest limit* on a one-element consumer array retires, since the file now tells the forms apart.

**Refused.** A reserved keyed knob, `DRIFT_KIT_PLUGIN_ENV[<NAME>] = value`. It changes the spelling
of every consumer's plugin knob and the plugin contract a consumer's plugins already read.

### (5) Defaults that read the layout, the filesystem or the harness home {design-bearing}

**The gates directory is a bridged input.** `CONTEXT_KIT_SETTINGS_PINS`, `CONTEXT_KIT_PUB_LANG_DIR`,
`DELEGATION_KIT_GATE_FILES`, `DELEGATION_KIT_META_PATHS`, `DRIFT_KIT_KPIS_FILE`,
`DRIFT_KIT_GATES_FILE`, `DRIFT_KIT_PRICE_TABLE` and `DRIFT_KIT_KPI_DIRS` derive from
`GATE_SDK_GATES_DIR` through cut 2's declared-input mechanism, beside the rows deriving from
`GATE_SDK_WORKFLOW_DIR`, `GATE_SDK_TMP_DIR` and `GATE_SDK_QUEUE_FILE`. `GATE_SDK_GATES_DIR` already
crosses the bridge (`--emit-drift-report` declares it). A static sibling input needs nothing new:
`DRIFT_KIT_OVERHEAD_LOG` and `DRIFT_KIT_STAGE_ECONOMICS_LOG` read `DRIFT_KIT_METRIC_DIR`.

**Two defaults probe the filesystem, and the roster must not print a probe of a placeholder.**

- `DRIFT_KIT_KPIS_FILE` is `<gates-dir>/kpis.list` when that file exists, else empty.
- `CONTEXT_KIT_HOOK_CMD` is `bash <candidate> --emit queue-index --collapse-deferred` for the first
  existing candidate among `<gates-dir>/run-gates.sh` and `<gate-sdk-root>/bin/run-gates.sh`, else
  empty. The second candidate reads `GATE_SDK_ROOT_HERE`, the bridged relative gate-sdk root, which
  replaces the library's own `BASH_SOURCE` location.

gate-sdk/SPEC.md §The knob file, the derived-default paragraph. **Not yet applied:**

> `--emit knob-roster` hands a derivation each bridged input as a **placeholder**, origin
> `Placeholder` and value `${NAME}`. A derivation that probes the filesystem does not probe a path
> built from a placeholder, and renders its first candidate instead, so the roster prints
> `${GATE_SDK_GATES_DIR}/kpis.list` rather than an empty value that is true of no tree.

**The harness home is derived at the reader.** `DELEGATION_KIT_USAGE_FILE` defaulted to
`${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt` and `DELEGATION_KIT_ACCOUNT_CONFIG` to
`$HOME/.claude.json`. Neither environment name is a knob, and a table row cannot declare one as an
input without the closure bridging it. Both default **empty**, and empty means *derive it*:
`native/src/hook/usage.rs` gains `paths()`, which reads each knob and fills an empty one from the crate's
one config-home derivation in `native/src/sessions.rs`, the one `DRIFT_KIT_SESSIONS_DIR` already
uses. `DELEGATION_KIT_CRED_FILE` stays a derived row reading its `DELEGATION_KIT_USAGE_FILE` sibling:
it is `<dir>/.credentials.json` for a set usage file and empty otherwise, and `paths()` fills an
empty one beside the derived usage file. The statusline, the poller and `--usage-verdict` read the
three paths through `paths()` alone.

**Refused.** An *ambient environment* input class for derived rows, so `HOME` could be declared. It
is a third input kind for two knobs that the empty-means-derive shape already serves, and the roster
would have to render a `:-` fallback the grammar has no spelling for.

### (6) Set-ness and retired-name refusals {design-bearing}

`CONTEXT_KIT_SETTINGS_FILE` and `DRIFT_KIT_KPIS_FILE` refuse at exit 2 when explicitly set to a path
that does not exist. The shell libraries took that refusal where set-ness was visible, and a static
resolution keeps set-ness as the value's origin. gate-sdk/SPEC.md §The knob file, the validator
paragraph. **Not yet applied:**

> A validator reads each row's origin beside its value, so a rule that turns on whether a consumer
> set the knob, not on what the value is, is a validator rule.

`knobs::Values` carries `(Value, Origin)`. The context-kit and drift-kit validators refuse an
existing-file knob whose origin is the environment, the local overlay or the tracked file and whose
path is not a file. drift-kit gains a validator for that rule alone. At its default,
`DRIFT_KIT_KPIS_FILE`'s empty not-adopted value is unchanged.

**A retired name is refused, naming its replacement.** `lib/context.sh` refused
`CONTEXT_KIT_BREVITY_SECTION` when set. gate-sdk/SPEC.md §The knob file, the refusal list. **Not yet
applied:**

> A kit's table may list **retired names**, each with its replacement. A file line naming one, or a
> retired scalar exported in the environment, is refused at exit 2 with the replacement named. The
> generic undeclared-name refusal would name only the roster, and an exported retired name would
> otherwise be silently ignored (the retired `GATE_SDK_GRAPH_THEME` precedent).

### (7) Two compositions move to their readers {design-bearing}

Both follow cut 2's icebox precedent: a composition the library applied **after** sourcing the
consumer config survives a consumer who set the base knob. A static file replaces a knob whole, and
a derived default fires only at the default, so the composition moves to the reader.

- **canon-kit's `_EXTRA` unions.** `native/src/spec.rs` gains `vocabulary(base, extra)`: the resolved
  base followed by the resolved extra. `check-manifest-temporal` reads `CANON_KIT_TEMPORAL_MARKERS`
  through it, and `check-prose-tells` reads `CANON_KIT_PROSE_TELL_PHRASES` and
  `CANON_KIT_PROSE_TELL_ABBR_ALLOW` through it, replacing its direct `walk::knob_array` reads. Each
  declares the matching `_EXTRA` knob.
- **`DELEGATION_KIT_META_PATHS`' kit-root union.** `check-gate-tamper` appends each
  `GATE_KIT_ROOTS_REL` member as a `<root>/` prefix the resolved value does not already hold, and
  declares `GATE_KIT_ROOTS_REL`. The `gate.sh`-resolvable condition goes: the crate always carries the
  root set, so *without `gate.sh` the config is used exactly as written* has no case left.

### (8) The templates {design-bearing}

- **Four `.knobs` templates.** `templates/{canon,context,delegation,drift}-config.sh` become
  `templates/*-config.knobs`, comment-only, each a `#` pointer to its kit's roster. The delegation
  template keeps its one commented example as `# DELEGATION_KIT_READONLY_TYPES[] = <agent-type>`.
  Init's seeding derivation and `check-template-copy-parity`'s suffix exclusion already take
  `*-config.knobs` (cut 2), so neither changes.
- **`drift-kit/templates/kpi-deprecated-surface.sh`** sourced `<gates-dir>/canon-config.sh` and read
  two canon-kit arrays as bash arrays. After this cut the file is absent, so the template would print
  its `n/a` row on every tree: the silent degradation a legacy refusal exists against. It reads both
  knobs from `run-gates.sh --emit knob-values CANON_KIT_DEPRECATION_MARKERS CANON_KIT_COMMENT_SURFACE`
  instead, finding the front-end under the `gate-sdk` member of the `DRIFT_KIT_KIT_ROOTS` handoff. A
  failed arm prints a fail-visible `n/a (knob read failed)` row, since a plugin never blocks. It stays
  a shell template on its `# no-port:` ground, and the header's *by sourcing a consumer config*
  becomes *through the knob-values arm*.

### (9) Delete the libraries, move the configs and fixtures {mechanical}

- Delete `canon-kit/lib/spec.sh`, `context-kit/lib/context.sh`, `delegation-kit/lib/delegation.sh`
  and `drift-kit/lib/drift.sh`, and each `lib/` directory left empty. No shell caller of any function
  or global in them remains: canon-kit's documented dead twins (the manifest finder, the amendment
  finder, the default-statement grammar) have compiled holders in `native/src/spec.rs`, and delta 10
  deletes the paragraphs that kept the twins.
- `git mv` the four `scripts/*-config.sh` to `.knobs`, rewritten to the grammar. Each
  `comment-tier-exempt:` or `spec:` comment survives as a `#` line, and the `# shellcheck` and
  `# no-port:` lines go with the substrate. `scripts/drift-config.knobs` carries delta 3's reference
  and `DRIFT_KIT_ICEBOX_SECTION = Icebox`.
- **Fixtures and tests.** Every fixture-tree `*-config.sh` of the four kits becomes `.knobs`: sixteen
  under `canon-kit/gate-tests/`, ten of them writing a command knob, and four under
  `context-kit/gate-tests/`. Every `*.test.sh`, smoke or crate test writing one of the four kits'
  config sets `<KIT>_KNOB_FILE` where it set `<KIT>_CONFIG_FILE`, and writes array and command values
  as `NAME[] =` lines. A test exporting a command knob or an array in the environment writes a knob
  file instead, because indexed knobs take no environment override. That includes
  `native/src/usage_tests.rs`, `native/src/emit/run_index_tests.rs` and
  `delegation-kit/smoke/install.sh`'s `DELEGATION_KIT_LIVENESS_CMD`. `drift-kit/smoke/install.sh`'s
  consumer knob `DRIFT_KIT_SMOKE_CUSTOM` becomes a scalar line in its knob file, and delta 4 keeps its
  assertion true. `scripts/gate-tests/subagent-stop-reader.test.sh` points `DELEGATION_KIT_KNOB_FILE`
  at `scripts/delegation-config.knobs`. `context-kit/smoke/install.sh` writes its config the same
  way. The AGENTS.md adapter smoke, `native/src/emit/agents_md_smoke.rs`, writes
  `scripts/canon-config.sh`, appends to the seeded `scripts/context-config.sh` and exports
  `CANON_KIT_CONFIG_FILE` into every battery run it spawns. It writes the `.knobs` bodies, appending
  to the seeded context template, and exports `CANON_KIT_KNOB_FILE`. The roster is
  `git grep -l -e CANON_KIT_CONFIG_FILE -e CONTEXT_KIT_CONFIG_FILE -e DELEGATION_KIT_CONFIG_FILE -e
  DRIFT_KIT_CONFIG_FILE -e canon-config -e context-config -e delegation-config -e drift-config`, run
  at build.
- `gate-sdk/gate-tests/run-arm-contract.test.sh` uses a scratch `canon-config.sh` as its example of a
  bridged config narrowing a knob. It takes evidence-kit, which stays bridged.
- **Registry declarations** (`native/src/gates/mod.rs`, `native/src/emit/mod.rs`): the ten output
  arrays leave every member; the two `_EXTRA` readers and `check-gate-tamper` gain delta 7's names;
  and the two `DRIFT_KIT_CONFIG_FILE` declarations go (delta 1).
- **Re-pointed mentions:** `canon-kit/README.md`, `context-kit/README.md`, `delegation-kit/README.md`
  and `drift-kit/README.md`'s config-copy steps; `.claude/commands/release-sweep.md`;
  `docs/site-architecture.md`; `doctrine-kit/SPEC.md` and `context-kit/SPEC.md`'s mentions of
  `canon-config.sh`; gate-sdk/SPEC.md's `check-gate-tamper` conservation row, which names
  `scripts/delegation-config.sh`; and the `# no-port:` headers of `scripts/install-transports.sh`,
  `scripts/measured-claims.sh` and `scripts/payload-claims.sh`, which cite `canon-config.sh`.
  `native/src/hook/mod.rs`' comment on why `agent-dispatch-guard` does not declare
  `DELEGATION_KIT_CONFIG_FILE` is deleted: a static kit's locator is read by the reading process
  itself, so nothing is left to explain.
- Regenerate the pre-commit hook, whose baked argv loses every `CANON_KIT_`, `CONTEXT_KIT_`,
  `DELEGATION_KIT_` and `DRIFT_KIT_` assignment, and the graph, the kit SPEC mirrors and the knob
  roster's readers.

### (10) The kits' own SPEC sections {design-bearing}

- **canon-kit/SPEC.md §Layout and configuration.** It states that the kit's knobs are static, as
  site-kit/SPEC.md does: the knob file, `CANON_KIT_KNOB_FILE`, the `.local.knobs` overlay and a pointer
  to gate-sdk/SPEC.md §The knob file. The *permanently shell* sentence goes. The five command bullets
  state the argv shape, and the transport, payload and claim-class bullets lose their output-pair
  sentences. The `_EXTRA` paragraph names the reader-side union (delta 7).
  `CANON_KIT_MEASURED_SURFACE_GLOBS`' *this repo sets* sentence spells the reference (delta 3), and
  its *deriving this knob from the manifest array is the common shape* paragraph keeps its warning.
- **canon-kit/SPEC.md §lib/spec.sh becomes §The shared spec adapters**, describing
  `native/src/spec.rs` as the sole holder. The emitter-backed-vocabularies bullet states the in-crate
  spawn, the per-process cache and the two validation contracts. Its nested-bridge paragraph keeps
  the termination constraint in delta 2's form, and keeps the residue: the spawned front-end still
  sources `lib/gate.sh`. The `_spec_resolving` membership text, the dead-twin paragraphs (the manifest
  finder, the amendment finder, the default-statement grammar) and the *permanently shell* paragraph
  are deleted, because their subjects are. The `// spec:` citations to the old heading follow the
  rename (`native/src/walk.rs` and the three `native/src/gates/amendment_*.rs` modules), and so do
  the citations to §lib/context.sh (`native/src/emit/pub_index.rs`) and §lib/drift.sh
  (`gate-sdk/lib/gate.sh`), each re-pointed at the section its rule now lives in.
- **context-kit/SPEC.md §Layout and configuration** states the kit static. §lib/context.sh's
  surviving rules fold into the roster bullets they govern, and the section goes. Those rules are
  `CONTEXT_KIT_MEMORY_DIRS` and `CONTEXT_KIT_PUB_LANGS`' empty-means-derive, and
  `CONTEXT_KIT_HOOK_CMD`'s two candidates, probe and empty answer as a derived row. The
  *repo-relative because a bridged value is baked into the hook* paragraph goes: a static value is
  never baked, and no default the table carries is absolute. `CONTEXT_KIT_BREVITY_SECTIONS` names the
  retired-name table and `CONTEXT_KIT_SETTINGS_FILE` the origin-reading validator (delta 6).
- **delegation-kit/SPEC.md §Layout and configuration** states the kit static, and the *permanently
  shell* paragraph goes. `DELEGATION_KIT_USAGE_FILE`, `_CRED_FILE` and `_ACCOUNT_CONFIG` state
  empty-means-derive (delta 5). `DELEGATION_KIT_REFRESH_CMD` and `_LIVENESS_CMD` state the argv shape
  (delta 2). `DELEGATION_KIT_META_PATHS` states the reader-side union (delta 7), and
  `DELEGATION_KIT_GATE_FILES`' *the loader guards it with `declare -p`* becomes *a file value
  replaces the default whole*. §The turn-end liveness hook's override paragraph follows delta 2.
- **drift-kit/SPEC.md §lib/drift.sh** is deleted. Its surviving rules move into §Layout and
  configuration's bullets: `DRIFT_KIT_KPIS_FILE`'s two adoption modes (delta 6) and
  `DRIFT_KIT_TRAJECTORY_SURFACES`' scalar two-field shape. The sole-resolver and kfric-duplicate
  records go with their subject. §Layout states the kit static and the open family (delta 4).
  `DRIFT_KIT_STAGES` teaches the reference (delta 3). `DRIFT_KIT_SESSIONS_DIR`'s *`lib/drift.sh`
  declares it empty so `declare -p` can find it* becomes *its default is empty*. §The KPI plugin
  contract reads the family (delta 4).

### (11) gate-sdk's standing rules and the release declarations {mechanical}

- **gate-sdk/SPEC.md §The knob file:** the `<stem>` examples gain the four stems. The shape list
  keeps the generated family, guard-kit's rule content and gate-sdk-last, and loses the two ruled
  shapes into the grammar (deltas 2 and 3). The refusal list gains the open family and retired names
  (deltas 4 and 6), the derived-default paragraph gains the placeholder (delta 5), and the validator
  paragraph gains origins (delta 6).
  **The transitional paragraph on a bridged config reading a static knob stays**, now with no live
  instance in this repo. `gate_static_knob` and `GATE_SDK_RESOLVING_KNOB` both remain until gate-sdk's
  cut: an adopter's still-bridged config may call the helper under that gating, and dropping the
  export would silently turn such a call off.
- **gate-sdk/SPEC.md §lib/gate.sh:** the `GATE_SDK_RESOLVING_KNOB` bullet names its one sanctioned
  reader, the transitional shape, and drops canon-kit's vocabularies as its instance.
- **gate-sdk/SPEC.md §The config-seam port disposition:** the four kits join the static set. Its
  *the ground is not minted here* paragraph cites `drift-kit/templates/drift-config.sh`'s own
  declaration as the class's precedent, and that file is replaced. The paragraph states the ground
  directly and names evidence-kit's template, still shell, as the live declarer.
- **gate-sdk/SPEC.md §The kit-library port disposition:** the member-level condition has fired for
  the four kits. The drift-kit precedent paragraph cites a deleted file, so it is re-grounded the same
  way.
- **`.workflow/release-declarations.md`.** **Not yet applied:**
  - Renamed knobs: `CANON_KIT_CONFIG_FILE` → `CANON_KIT_KNOB_FILE`, `CONTEXT_KIT_CONFIG_FILE` →
    `CONTEXT_KIT_KNOB_FILE`, `DELEGATION_KIT_CONFIG_FILE` → `DELEGATION_KIT_KNOB_FILE`,
    `DRIFT_KIT_CONFIG_FILE` → `DRIFT_KIT_KNOB_FILE`. The ten canon-kit output arrays → ∅, since they
    were never set by a consumer and nothing reads them. The existing `CONTEXT_KIT_BREVITY_SECTION`
    bullet's *`lib/context.sh` refuses the retired name* becomes *the knob-file reader refuses it*.
  - Behavior changes: **`<gates-dir>/{canon,context,delegation,drift}-config.sh`** are replaced by
    `.knobs` files and the four kit libraries are deleted, with defaults printed by `run-gates.sh
    --emit knob-roster`. A left-behind shell config is refused at exit 2, **including the copy an
    earlier `init` seeded that you never edited**: delete it, or rewrite what you set into the
    `.knobs` file `init` now seeds. An exported scalar knob of these kits now outranks the file.
    **Every `*_CMD` knob is an argv**, one `NAME[] = element` line per word, and runs with no shell:
    `CANON_KIT_*_CMD`, `CONTEXT_KIT_HOOK_CMD` and `DELEGATION_KIT_REFRESH_CMD` values that relied on
    shell syntax need a `bash`, `-c` and command-string argv, and none takes an environment override.
    `DELEGATION_KIT_LIVENESS_CMD` may now name an interpreter. A `drift-config.sh` that called
    `gate_static_knob LIFECYCLE_KIT_STAGES DRIFT_KIT_STAGES` writes `DRIFT_KIT_STAGES[] <-
    LIFECYCLE_KIT_STAGES` instead. **`drift-kit/templates/kpi-deprecated-surface.sh`** reads
    canon-kit's knobs through `--emit knob-values`, so a copy you adapted from the old template reads
    nothing after the upgrade: re-copy the template.

## Producers and consumers

- **The four defaults tables and three validators** (new state). *Producer:*
  `native/src/knobs/{canon,context,delegation,drift}_kit.rs`. *Consumers:* `knobs::resolve` for
  every read of the four prefixes, `--emit knob-roster`, `--emit knob-values`, the undeclared-name
  refusal, and each validator at its kit's first resolution.
- **The command-knob spawn** (changed interface). *Producer:* each reader, through the crate's program
  resolver. *Consumers:* the eight knobs' readers named in delta 2. Enabling config: this repo sets
  seven of the eight, and a consumer that sets no `CONTEXT_KIT_HOOK_CMD` reaches its probed default.
  `DELEGATION_KIT_LIVENESS_CMD` is set only by `delegation-kit/smoke/install.sh`, and the hook's
  compiled default runs everywhere else.
- **The canon-kit output adapters** (new interface, replacing ten knobs). *Producer:*
  `native/src/spec.rs`, once per process per command. *Consumers:* `check-prose-enum`,
  `check-install-claim`, `check-payload-claim`, `check-measured-claim` and `check-unmarked-claim`.
  Each parsed line's two fields are read by its member, the first as set name, id or key and the
  second as member, pattern or value.
- **`Form::Reference`** (new state). *Producer:* `knobfile::parse`. *Consumers:* `knobs::lookup`,
  which splices it, and `registry::knob_files`, which reads the referent's owner. The referent field
  is read at both transitions.
- **`Kit.open_family` and `knobs::family`** (new field and interface). *Reader of the field:*
  `knobs::layer`, which admits a consumer scalar rather than refusing it. *Consumer of the family:*
  `--emit-drift-report`'s `child_env`, which exports it to plugins, where
  `drift-kit/smoke/install.sh` reads it.
- **`Origin::Placeholder`** (new value). *Producer:* `knobs::roster`'s `defaults_only`. *Readers:*
  `DRIFT_KIT_KPIS_FILE`'s and `CONTEXT_KIT_HOOK_CMD`'s derivations, at roster time only.
- **`Values`' origin** (new field). *Readers:* the context-kit and drift-kit validators, at first
  resolution.
- **`Kit.retired`** (new field). *Readers:* `knobs::layer`'s undeclared-name refusal and the kit
  validator's environment check.
- **`spec::vocabulary`** and **`check-gate-tamper`'s union** (new compositions). *Consumers:* the three
  base-vocabulary readers, and assertion A's meta-layer test.
- **`usage::paths`** (new interface). *Consumers:* `native/src/hook/statusline.rs`,
  `native/src/hook/poll.rs` and `native/src/hook/verdict.rs`.
- **Red conditions (point 5).** Delta 9 narrows several corpora by deleting four libraries, four
  shell configs and four templates, and delta 2 retires ten names.
  - *`check-docs-cmd` assertion B's known-knob set* reds on a documented name absent from the set.
    Deleting the libraries removes kit-root source mentions, which can **add** reds, so the reader is
    not monotone. The static roster covers every declared row. A documented name that was only a
    library global, such as the ten arrays, reds unless delta 10 removed its mention. Build runs the
    gate.
  - *`check-knob-default-coupling`* reds on a SPEC default that disagrees with its source, and on a
    source default the SPEC does not state. The migrated scalars move to the roster idiom. A
    derived or probing default renders `${NAME}` and classifies as computed, so it is skipped and
    counted. The skipped count moves, and nothing asserts it.
  - *§port-blockers' `--tree` counts and every measured claim over them* read exact counts. Four
    libraries, four consumer configs and four templates leave the scanned shell set, and
    `kpi-deprecated-surface.sh` stays. `check-measured-claim` re-runs its oracles, and build runs the
    battery rather than inspecting.
  - *`check-comment-tier`* reds on an ungoverned comment. `CANON_KIT_COMMENT_SURFACE` already globs
    `*.knobs`, so the four new consumer files come under it as the `.sh` files leave. The comments
    move verbatim with their directives. Build runs it.
  - *`check-graph` assertion D* reds on a stale hook. Every member reading one of the four prefixes
    loses baked argv elements, and delta 3 adds a derived trigger. Delta 9 regenerates.
  - *`check-reads-couples` and `check-gate-substrate-parity`* compare `--knobs` against couples and
    the registry. The answer narrows for every member of the four kits, and no `knob:` token they
    carry leaves its member's declaration, so the admissibility assertion stays green. Build runs
    both and states which held.
  - *`check-template-copy-parity`* pairs templates with copies. Four `.sh` exclusions leave and four
    `.knobs` exclusions arrive, so the net finding set is unchanged by inspection. Build runs it.
  - *`check-kit-ref-liveness`* reds on a kit knob that resolves to no tracked kit knob. The ten
    retired names must leave every scanned surface, and `## Retired spellings` holds that.
  - *`drift-kit/smoke/install.sh`* reds when a plugin misses a consumer knob. Delta 4 keeps the knob
    exported. Build runs the consumer smoke.
- **SPEC-knob-file-couples.md's oracle, lifecycle-kit row.** After delta 3, `run-gates.sh --for
  scripts/lifecycle-config.knobs` must select `check-trajectory-fresh`.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The knob file (deltas 2, 3, 4, 5, 6 and 11); §lib/gate.sh, the
  `GATE_SDK_RESOLVING_KNOB` bullet (delta 11); §The config-seam port disposition and §The
  kit-library port disposition (delta 11); §The `# graph:` manifest, the derived knob-file couple
  (delta 3); §Meta-gate conservation, the `check-gate-tamper` row (delta 9).
- `canon-kit/SPEC.md` — §Layout and configuration and §lib/spec.sh renamed (deltas 2, 3, 7 and 10);
  §check-prose-enum, §check-install-claim, §check-payload-claim, §check-measured-claim and
  §check-unmarked-claim, where each names its output pair (delta 2).
- `context-kit/SPEC.md` — §Layout and configuration and §lib/context.sh (deltas 5, 6 and 10); its
  `canon-config.sh` mentions (delta 9).
- `delegation-kit/SPEC.md` — §Layout and configuration and §The turn-end liveness hook (deltas 2, 5,
  7 and 10); §usage-verdict, the refresh spawn (delta 2).
- `drift-kit/SPEC.md` — §lib/drift.sh deleted, §Layout and configuration, §The KPI plugin contract
  (deltas 3, 4, 6 and 10); §Out of scope, the example template (delta 8).
- `doctrine-kit/SPEC.md` — the `canon-config.sh` mention (delta 9).
- `canon-kit/README.md` — config step (delta 9).
- `context-kit/README.md` — config step (delta 9).
- `delegation-kit/README.md` — config step (delta 9).
- `drift-kit/README.md` — config step (delta 9).
- `native/src/knobs/mod.rs` — `STATIC_KITS`, references, the open family, the placeholder origin,
  `Values`' origin, retired names (deltas 1, 3, 4, 5 and 6).
- `native/src/knobs/canon_kit.rs` — new table and validator (delta 1).
- `native/src/knobs/context_kit.rs` — new table and validator (deltas 1, 5 and 6).
- `native/src/knobs/delegation_kit.rs` — new table and validator (deltas 1 and 5).
- `native/src/knobs/drift_kit.rs` — new table and validator (deltas 1, 4, 5 and 6).
- `native/src/knobfile.rs` — the reference form (delta 3).
- `native/src/registry.rs` — the reference reach in `knob_files` (delta 3).
- `native/src/spec.rs` — the command adapters and `vocabulary` (deltas 2 and 7).
- `native/src/hook/usage.rs` — `paths` (delta 5).
- `native/src/gates/mod.rs` — declarations (delta 9).
- `native/src/emit/mod.rs` — declarations (delta 9).
- `native/src/gates/prose_enum.rs` — the enum-set adapter (delta 2).
- `native/src/gates/install_claim.rs` — the vocabulary adapter (delta 2).
- `native/src/gates/payload_claim.rs` — the vocabulary adapter (delta 2).
- `native/src/gates/measured_claim.rs` — the measured-claim adapter (delta 2).
- `native/src/gates/unmarked_claim.rs` — the vocabulary adapter (delta 2).
- `native/src/gates/manifest_temporal.rs` — the union (delta 7).
- `native/src/gates/prose_tells.rs` — the union (delta 7).
- `native/src/gates/gate_tamper.rs` — the kit-root union (delta 7).
- `native/src/emit/always_loaded.rs` — the argv spawn (delta 2).
- `native/src/emit/drift_report.rs` — the family read (delta 4).
- `native/src/hook/verdict.rs` — the argv spawn and `usage::paths` (deltas 2 and 5).
- `native/src/hook/stop_liveness.rs` — the argv spawn (delta 2).
- `native/src/hook/statusline.rs` — `usage::paths` (delta 5).
- `native/src/hook/poll.rs` — `usage::paths` (delta 5).
- `native/src/usage_tests.rs` — knob-file writers (delta 9).
- `native/src/emit/run_index_tests.rs` — knob-file writers (delta 9).
- `canon-kit/lib/spec.sh` — deleted (delta 9).
- `context-kit/lib/context.sh` — deleted (delta 9).
- `delegation-kit/lib/delegation.sh` — deleted (delta 9).
- `drift-kit/lib/drift.sh` — deleted (delta 9).
- `canon-kit/templates/canon-config.sh` — replaced by the `.knobs` template (delta 8).
- `context-kit/templates/context-config.sh` — replaced by the `.knobs` template (delta 8).
- `delegation-kit/templates/delegation-config.sh` — replaced by the `.knobs` template (delta 8).
- `drift-kit/templates/drift-config.sh` — replaced by the `.knobs` template (delta 8).
- `drift-kit/templates/kpi-deprecated-surface.sh` — the knob-values read (delta 8).
- `scripts/canon-config.sh` — moved to `.knobs`, with the same-kit reference (deltas 3 and 9).
- `scripts/context-config.sh` — moved to `.knobs` (delta 9).
- `scripts/delegation-config.sh` — moved to `.knobs` (delta 9).
- `scripts/drift-config.sh` — moved to `.knobs`, with the cross-kit reference (deltas 3 and 9).
- `canon-kit/gate-tests/` — config writers (delta 9).
- `context-kit/gate-tests/` — config writers (delta 9).
- `context-kit/smoke/install.sh` — config writers (delta 9).
- `native/src/emit/agents_md_smoke.rs` — the adapter smoke's config writers and battery pin (delta 9).
- `native/src/hook/mod.rs` — the locator comment (delta 9).
- `native/src/walk.rs` — section citation (delta 10).
- `native/src/gates/amendment_queue.rs` — section citation (delta 10).
- `native/src/gates/amendment_update_target.rs` — section citation (delta 10).
- `native/src/gates/amendment_retired_spelling.rs` — section citation (delta 10).
- `native/src/emit/pub_index.rs` — section citation (delta 10).
- `gate-sdk/lib/gate.sh` — section citation (delta 10).
- `scripts/install-transports.sh` — header citation (delta 9).
- `scripts/measured-claims.sh` — header citation (delta 9).
- `scripts/payload-claims.sh` — header citation (delta 9).
- `delegation-kit/smoke/install.sh` — config writers (delta 9).
- `drift-kit/smoke/install.sh` — config writers and the consumer knob (deltas 4 and 9).
- `scripts/gate-tests/subagent-stop-reader.test.sh` — the knob-file pin (delta 9).
- `gate-sdk/gate-tests/run-arm-contract.test.sh` — the bridged example (delta 9).
- `.claude/commands/release-sweep.md` — config path (delta 9).
- `docs/site-architecture.md` — config path (delta 9).
- `scripts/git-hooks/pre-commit` — regenerated (deltas 3 and 9).
- `.workflow/release-declarations.md` — Renamed knobs and Behavior changes (delta 11).
- `docs/check-graph.html` — generated artifact, regenerated (deltas 3 and 9).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/canon-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/context-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/delegation-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/doctrine-kit/SPEC.md` — generated mirror, regenerated (delta 9).
- `docs/canon-kit/README.md` — generated mirror, regenerated (delta 9).
- `docs/context-kit/README.md` — generated mirror, regenerated (delta 9).
- `docs/delegation-kit/README.md` — generated mirror, regenerated (delta 9).
- `docs/drift-kit/README.md` — generated mirror, regenerated (delta 9).
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the library it shipped stays -->
- `docs/posts/2026-08-14-checkwright-v0-23-0.md` — unchanged.
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the library it shipped stays -->
- `docs/posts/2026-08-21-checkwright-v0-24-0.md` — unchanged.

## Retired spellings

- `canon-kit/lib/spec.sh` — deleted (delta 9).
- `context-kit/lib/context.sh` — deleted (delta 9).
- `delegation-kit/lib/delegation.sh` — deleted (delta 9).
- `drift-kit/lib/drift.sh` — deleted (delta 9).
- `canon-config.sh` — replaced by `canon-config.knobs` (deltas 8 and 9).
- `context-config.sh` — replaced by `context-config.knobs` (deltas 8 and 9).
- `delegation-config.sh` — replaced by `delegation-config.knobs` (deltas 8 and 9).
- `drift-config.sh` — replaced by `drift-config.knobs` (deltas 8 and 9).
- `CANON_KIT_CONFIG_FILE` — renamed `CANON_KIT_KNOB_FILE` (delta 11).
- `CONTEXT_KIT_CONFIG_FILE` — renamed `CONTEXT_KIT_KNOB_FILE` (delta 11).
- `DELEGATION_KIT_CONFIG_FILE` — renamed `DELEGATION_KIT_KNOB_FILE` (delta 11).
- `DRIFT_KIT_CONFIG_FILE` — renamed `DRIFT_KIT_KNOB_FILE` (deltas 1 and 11).
- `CANON_KIT_ENUM_SET_NAMES` — retired output array (delta 2).
- `CANON_KIT_ENUM_SET_MEMBERS` — retired output array (delta 2).
- `CANON_KIT_MEASURED_KEYS` — retired output array (delta 2).
- `CANON_KIT_MEASURED_VALUES` — retired output array (delta 2).
- `CANON_KIT_INSTALL_TRANSPORT_IDS` — retired output array (delta 2).
- `CANON_KIT_INSTALL_TRANSPORT_PATTERNS` — retired output array (delta 2).
- `CANON_KIT_PAYLOAD_CLAIM_IDS` — retired output array (delta 2).
- `CANON_KIT_PAYLOAD_CLAIM_PATTERNS` — retired output array (delta 2).
- `CANON_KIT_CLAIM_CLASS_IDS` — retired output array (delta 2).
- `CANON_KIT_CLAIM_CLASS_PATTERNS` — retired output array (delta 2).
- `_spec_resolving` — the gating helper, deleted with its library (delta 2).
- `§lib/spec.sh` — section renamed §The shared spec adapters (delta 10).
- `§lib/context.sh` — section folded into §Layout and configuration (delta 10).
- `§lib/drift.sh` — section deleted, its rules moved to §Layout and configuration (delta 10).

## Definition of Done

- [ ] **Causal completeness** — every new state and interface above has a reachable producer and a
      named consumer; the reference refusals, the open family, the placeholder and the origin-reading
      validators each have a unit test.
- [ ] **Instruction surfaces: instruction only** — the refusal messages, the templates and the release
      declaration carry the remedy, not the grounds.
- [ ] **Merged with no information lost** — each surviving library rule lands in its roster bullet or
      adapter section. Deleted argument prose (sole resolver, permanently shell, dead twins, resolving
      membership, bridged repo-relativity) is gone because its subject is, and the refused
      alternatives stay in this file's history.
- [ ] **Amendment deleted**, and the entry is **demoted, not moved to Done** (corpus increment),
      restoring its position and board tags from this promotion's diff and compressing it under the
      entry cap.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any reader delta 9's grep roster missed, the set-but-missing refusal fixtures
      the census found absent for `CONTEXT_KIT_SETTINGS_FILE` and `DRIFT_KIT_KPIS_FILE` if build does
      not add them with delta 6, and whichever assertion the retired names reddened, if it lands as a
      gap.
