# SPEC amendment: knob-files-cut-4

Queue entries: `config-seam-static-format`, its fourth increment, beside
`append-and-wrapper-shapes-unsteered` and `upgrade-smoke-consumer-unseeded-configs`. They lead unit
set `config-seam-fourth-cut`: **cut 4 is evidence-kit, then guard-kit**, and it rules the two shapes
gate-sdk/SPEC.md §The knob file still lists as unruled for them, the generated family and guard-kit's
rule content (operator direction, 2026-09-14, lead-relayed). gate-sdk's own cut retires the bridge and
is SPEC-bridge-retirement.md, which lands after this file. The grammar, the precedence, the legacy
refusals, the command knob, the reference form, the open family, the retired-name table and the
origin-reading validator stand as cuts 1 to 3 ruled them. This amendment adds only what the two kits
need beyond them.

Scope's promotion left three things for spec to settle, and each is ruled here. The generated family
is deltas 2, 3 and 4. guard-kit's rule content is delta 7: the guard stays the one shell hook, and only
the ground that made its library the knobs' resolver dissolves. The append and wrapper steers are delta
9, and the seam question their entry asked resolves to kit mechanism, because the measured append class
needs no destination roster at all. The upgrade smoke's seeding is delta 1, and it lands first because
each cut here is a config-shape change that entry's cost names.

**Measured at `692ad8c2` for the bridge's residue (config-bridge-resolution-cost's probe, cited by
SPEC-bridge-retirement.md).** A bare battery's union resolution, `gate_knob_env --run --gates-dir
scripts`, takes 120 to 124 ms over five runs and emits 76 elements: 30 `EVIDENCE_KIT_`, 29 `GATE_SDK_`
and 17 resolved `GATE_*` and `GRAPH_*` globals. No `GUARD_KIT_` name is in the run union. Sourcing
`lib/gate.sh` alone takes 12 ms. The battery's per-gate timings sum to 30 664 ms at the same head.

**Measured for the guard hook at the same head.** `bash scripts/bash-guard.sh` on a `git status` payload
takes 58 to 63 ms per call over ten runs, dominated by bash start-up and the `jq` field reads.

## The seam

- **Kit mechanism:** the fixture-suite derivation and its arm, the declared-family and declared-referent
  rules in `native/src/knobs/mod.rs`, the two defaults tables and validators, the two comment-only
  `.knobs` templates, `lib/guard.sh`'s knob load, the upgrade smoke's seeding step, and guard rules 25
  and 26.
- **Consumer config:** each `<gates-dir>/{evidence,guard}-config.knobs` and its gitignored
  `.local.knobs` overlay, the adopter's edit seam, whose ground gate-sdk/SPEC.md §The config-seam port
  disposition states.
- **This repo's own:** `scripts/evidence-config.knobs` carries this repo's suite roster, its hand-listed
  runner commands and its two parser overrides; `scripts/guard-config.knobs` carries its breadth probes,
  its breadth declarations and its empty search-tool roster. The declarations' reasons are this
  project's operator rulings verbatim, and they stay consumer values, never kit literals.
- **Private rule content:** none newly in reach. Delta 9's steers name no destination and no command
  vocabulary, only the harness's own tool names, which §The generic ruleset already rules public.

## What changes

### (1) The upgrade smoke seeds what init seeds {design-bearing}

gate-sdk/SPEC.md §upgrade-smoke gains a step between the FROM install and the FROM baseline, and the
upgrade contract's phase A gains a step between the sync and the regen. **Widened at build** (operator
direction, 2026-09-14, lead-relayed): the seeding alone, built as first authored, made phase A's regen
refuse, because `gen-pre-commit` reads the knob values a `couples=knob:` token names and a seeded
`canon-config.sh` refuses that read at exit 2, which no declaration contains. So the contract now handles
a left-behind shell config and the suite goes green on it. **Applied at build, first batch:**

> **The FROM consumer is seeded with the config seam `init` derives.** After the FROM kits are vendored
> and each kit's `smoke/install.sh` has run, the arm copies every destination
> `recipe::config_seam_plan` derives over each vendored FROM kit root that is still absent in the
> scratch consumer, and commits the result into the baseline commit. So the baseline is the tree an
> init-seeded consumer holds, and a kit whose smoke seeds no config still has its template copy on disk.
> A smoke that wrote its own config keeps it, because a copy lands only where nothing is. A seeded copy
> that reds the FROM baseline is the tag's own defect, reported as `FAIL(env)` like every other red
> baseline.
>
> **Phase A retires the shell configs TO's kits replaced with knob files, before it regenerates.** A
> kit that moved to a knob file ships `templates/<stem>-config.knobs` and no `<stem>-config.sh`, and its
> loader refuses a `<gates-dir>/<stem>-config.sh` or `<stem>-config.local.sh` left beside it. The regen
> reads knob values, so it cannot run until those files are gone. The upgrade contract's phase A
> therefore has three steps: sync the kit directories, retire each such shell config (rewrite what you
> set into the `.knobs` file, then delete it), and regenerate. The arm performs the middle step by
> deleting each one, with the set derived through `recipe::config_seam_plan` over TO's kit roots. It
> runs after the determinism check, so that check still measures the sync alone. Deleting is one of the
> declared remedies, and the scratch consumer holds nothing an adopter set.
>
> A contract-following consumer therefore meets no red from a left-behind config, and the suite no
> longer requires a Tightened-gates bullet for one. A gate that reds because the deleted file held a
> value its kit smoke set still reaches phase B, where TO's declaration must contain it.
>
> **The honest limit.** Both derivations are the host's, over FROM's and TO's templates, which is exact
> while the rule is unchanged since FROM. The placement seam `init` writes beside an artifact
> (installer/README.md §The gate binary) is not seeded, because writing it faithfully means running
> FROM's installer, the cross-version init path this suite does not reach. The suite deletes rather than
> rewrites, so it never proves an adopter's rewrite.

`native/src/emit/upgrade_smoke.rs` gains the seeding step after `vendor_and_install` and the retirement
step between `determinism` and `regenerate`, both reusing `recipe::config_seam_plan`, the one owner of
the derivation. `gate-sdk/lib/consumer-smoke.sh` is not touched, so the consumer smoke is unchanged.
docs/install.md §The upgrade contract states the middle phase-A step, and
`lifecycle-kit/templates/upgrade.md` step 1 names it. Unit tests: a scratch kit root holding a `.sh` and
a `.knobs` template gets both copies, and an existing destination is left alone. A TO kit root shipping
only a `.knobs` template retires both shell spellings of its stem. A kit still shipping its `.sh`
template, and a stem no TO kit ships, retire nothing.

### (2) The fixture-suite derivation moves into the crate {design-bearing}

`gate_fixture_suites` computes the roster of fixture suites in shell. Its callers are this repo's CI
workflow, the shipped workflow template and this repo's evidence config loop, and delta 5 needs the
same roster in the crate. So the derivation gets one compiled holder and the shell function is deleted
in the same commit. **Applied at build, second batch.**

- **`registry::fixture_suites()`** returns one `(suite, tests_dir, checks_dir)` triple per directory
  carrying a `gate-tests/` tree: the kit roots in `GATE_KIT_ROOTS_REL` order, then the gates directory.
  The suite name is the directory's basename with `-` turned to `_`. The checks directory is the
  sibling `checks/` when it exists and empty otherwise. It reads the kit roots through the bridge today
  and through the crate's own derivation after SPEC-bridge-retirement.md.
- **`--emit fixture-suites`**, a non-gate arm, prints one `<suite>⇥<tests-dir>⇥<checks-dir>` line per
  triple, the shell function's exact rows. It declares `GATE_KIT_ROOTS_REL` and `GATE_SDK_GATES_DIR`.
- **The two workflows** (`.github/workflows/gates.yml` and `gate-sdk/templates/gates-workflow.yml`)
  capture the arm's output before looping, `suites="$(bash gate-sdk/bin/run-gates.sh --emit
  fixture-suites)" || exit 2`, and read the loop from that variable. A loop reading the arm through a
  process substitution would lose its status, and an arm failure would run zero suites and pass.

At build, before the shell function is deleted, the arm's output is compared with
`gate_fixture_suites`' in this tree and must be byte-identical. That is a port-time proof and holds no
standing twin, since the shell holder leaves in the same commit.

### (3) A kit may declare a scalar family {design-bearing}

gate-sdk/SPEC.md §The knob file, the generated-family bullet of the unruled-shapes list, becomes a rule.
**Applied at build, second batch**, with two calibrations merged beside it: the line grammar's name
takes an uppercase letter then letters, digits and `_`, so a member's lowercase suffix parses, and
the validator's read skips a declared referent's splice that reaches a bridged input:

> **A kit may declare a scalar family**, a prefix under which each member is a scalar whose name the
> prefix and a suffix spell: `EVIDENCE_KIT_RUN_demo = bash gate-sdk/bin/run-gates.sh --run-demo`. A
> file line naming an undeclared scalar under a declared family prefix is a family member rather than a
> refusal. The suffix is non-empty and SCREAMING_SNAKE-or-lowercase identifier-shaped
> (`[A-Za-z_][A-Za-z0-9_]*`), and an indexed or keyed line under the prefix is refused. A member resolves
> through the scalar precedence, so an exported `EVIDENCE_KIT_RUN_demo` outranks the file. A declared
> row whose name the prefix also spells, such as `EVIDENCE_KIT_RUN_ID`, is never a member.
>
> **A family may carry a derivation**, the generated family: a function of already-resolved knobs
> returning `(suffix, value)` members, with every name it reads declared like a derived row's. A derived
> member is the family's default for that suffix, so a file or environment member of the same name
> replaces it. It is a kit-side derivation and never a loop in consumer config.
>
> **The family is still a resolution set and never a roster.** A reader looks a member up by the
> suffix its own roster knob names, and `knobs::family(prefix)` returns every derived member, every
> member either file sets, and every member the environment exports, the environment first.
>
> The honest limit is the open family's, narrowed to one prefix: a misspelled member reads as a member,
> and it is found only by the reader whose roster names the correct spelling and does not find it.

`Kit` gains `families: &[Family]`, each with its prefix, an optional derivation and its declared
inputs. `knobs::layer` admits a family member where it now refuses an undeclared name, and
`walk::knob_prefix` reads `knobs::family` for a statically owned prefix instead of the declared rows
alone. `knobs::bridged` adds a family derivation's bridged inputs when a member declares the family's
`*` name, as it does a derived row's. Unit tests cover a file member, an environment member outranking
it, a derived member replaced by a file member, an indexed line under the prefix refused, and a declared
row excluded from its family.

**Refused.** The drift-kit open family for evidence-kit. It admits an undeclared scalar anywhere under
the kit's prefix, so every misspelled knob in the file would read as a consumer knob, where a declared
prefix confines that limit to the two families. Also refused: a keyed knob, `EVIDENCE_KIT_RUN[demo] =
…`. It renames every consumer's run and parser knobs, and gate-sdk/SPEC.md §lib/gate.sh already recorded
why the family is separate variables and not a map.

### (4) A row may declare the referents a reference to it may name {design-bearing}

gate-sdk/SPEC.md §The knob file, the reference refusal list. The bullet refusing a referent that reaches
a bridged input gains its exception. **Applied at build, second batch:**

> - `OTHER`'s row reaches a bridged input **and `NAME`'s row does not declare `OTHER` a referent**. A
>   row may list the referents a reference to it may name. The member's `--knobs` closure then adds a
>   declared referent's bridged inputs whenever the member reads `NAME`, so the bridge carries what the
>   splice needs. An undeclared referent is still refused, because the closure is computed from
>   declarations and cannot see a file's reference.

The live instance is delta 5's: `EVIDENCE_KIT_SUITES` declares `EVIDENCE_KIT_FIXTURE_SUITES` a referent,
and this repo writes `EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES`. `Row` gains `referents`,
`refuse_referent` reads it, and `bridged_inputs` follows it. The rule has no case left once
SPEC-bridge-retirement.md removes every bridged input. A unit test holds an undeclared referent reaching a
bridged input refused, and a declared one resolving with its input carried.

**Refused.** Reading a consumer file's references into the `--knobs` closure at query time. It makes a
member's bridge depend on file content the registry does not declare, so a hook baked from one tree
would carry another tree's inputs. Also refused: a hand-listed fixture-suite roster in this repo's file
until gate-sdk's cut. It is a maintained roster, and it goes stale silently if that cut does not land.

### (5) evidence-kit goes static {mechanical}

**Applied at build, second batch.**

evidence-kit gains `native/src/knobs/evidence_kit.rs` and joins `STATIC_KITS`. Every row transcribes
from `evidence-kit/lib/evidence.sh` as it stands at this amendment's commit:

- **Scalars:** `EVIDENCE_KIT_PARSER` (`exit-code`), `EVIDENCE_KIT_RUN_ID` and `EVIDENCE_KIT_PRE_HOOK`
  (empty), and `EVIDENCE_KIT_RUNNER_DOC` (`README.md`).
- **Derived scalars:** `EVIDENCE_KIT_BASELINE_FILE`, `_MANIFEST_FILE`, `_SKIP_FILE` and `_STATE_FILE`
  from `GATE_SDK_WORKFLOW_DIR`, `EVIDENCE_KIT_QUEUE_FILE` from `GATE_SDK_QUEUE_FILE`,
  `EVIDENCE_KIT_TMP_DIR` from `GATE_SDK_TMP_DIR`, and `EVIDENCE_KIT_LOCK_FILE` from its
  `EVIDENCE_KIT_TMP_DIR` sibling.
- **Indexed:** `EVIDENCE_KIT_SUITES` (empty, declaring delta 4's referent) and
  `EVIDENCE_KIT_PERMANENT_SLUGS` (empty). **Keyed:** `EVIDENCE_KIT_SCENARIO_GLOBS` (empty).
- **`EVIDENCE_KIT_FIXTURE_SUITES`**, a new derived indexed row, is the suite names
  `registry::fixture_suites()` returns (delta 2), declaring its bridged inputs. No reader reads it
  directly: it exists to be referenced into the roster, so an adopter who wants the derived suites
  splices them where the order wants them, and one who does not leaves it out.
- **Families (delta 3):** `EVIDENCE_KIT_RUN_` carries the derivation, one member per fixture suite,
  valued `bash <GATE_SDK_ROOT_HERE>/bin/run-gates.sh --run-gate-tests <tests-dir>` followed by
  ` <checks-dir>` when one exists: the value this repo's loop spells today. `EVIDENCE_KIT_PARSER_` carries
  none.
- **The validator** moves from the library whole: `EVIDENCE_KIT_PARSER`, `_BASELINE_FILE`,
  `_MANIFEST_FILE` and `_QUEUE_FILE` non-empty, and every `EVIDENCE_KIT_SUITES` element a valid family
  suffix. The three derived-from-bridged rows skip at their default, per the validator rule.

The run and parser values keep their contract: word-split, spawned with no shell, the parser's log path
appended last (evidence-kit/SPEC.md §lib/evidence.sh). They are scalars, not command knobs, because they
are not `_CMD`-named and their readers' contract is the word split, which cut 2's `QUEUE_KIT_LESSON_SINKS`
precedent keeps.

`--run-validate`'s declaration of `EVIDENCE_KIT_STATE_FILE`, which that arm never reads, is deleted.

### (6) evidence-kit's library, configs and fixtures {mechanical}

**Applied at build, second batch.** The roster grep missed two readers, both fixed in the same batch:
`gate-sdk/gate-tests/check-reads-couples.test.sh` case F, whose withheld bridged filter knob no longer
exists and which now drives a malformed static knob file, and canon-kit/SPEC.md §check-docs-cmd's
`GATE_SDK_LIB` example with its unit-test twin in `native/src/gates/docs_cmd.rs` and its fixture in
`canon-kit/gate-tests/check-docs-cmd/good/doc.md`, whose only kit-code reader was the deleted library.

- Delete `evidence-kit/lib/evidence.sh` and the empty `lib/` directory. Its five shell adapters have no
  caller anywhere in the tree, and each has its compiled holder in `native/src/evidence.rs`.
  `EVIDENCE_MANIFEST_CONTRACT` leaves with it, so `evidence::MANIFEST_CONTRACT` is the one holder and
  the test at `native/src/evidence.rs` that sources the library to compare the two is deleted.
- `git mv scripts/evidence-config.sh scripts/evidence-config.knobs`, rewritten to the grammar:
  `EVIDENCE_KIT_SUITES[] = gates`, `EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES`, then one
  element line per hand-listed suite in today's order, the two `EVIDENCE_KIT_PARSER_` overrides and the
  eight hand-listed `EVIDENCE_KIT_RUN_` members as scalar lines, and `EVIDENCE_KIT_RUN_gates` beside
  them. `EVIDENCE_KIT_PARSER = exit-code` is dropped as the default. The `spec:` comments survive as `#`
  lines, and the `# shellcheck` and `# no-port:` lines go with the substrate.
- `evidence-kit/templates/evidence-config.sh` becomes `templates/evidence-config.knobs`, comment-only, a
  `#` pointer to the kit's roster.
- **Fixtures and tests.** Every fixture-tree `evidence-config.sh` becomes `.knobs`: six under
  `evidence-kit/gate-tests/`. Every test writing an evidence config writes knob-file lines and sets
  `EVIDENCE_KIT_KNOB_FILE` where it set `EVIDENCE_KIT_CONFIG_FILE`. A test exporting a scalar keeps the
  export, since a scalar's environment override survives. A test that declared the keyed knob with
  `declare -A` writes `EVIDENCE_KIT_SCENARIO_GLOBS[<suite>] = <glob>` lines. The roster is
  `git grep -l -e EVIDENCE_KIT_CONFIG_FILE -e evidence-config -e 'lib/evidence.sh'`, run at build. It
  includes `evidence-kit/smoke/install.sh`, `scripts/gate-tests/evidence-parser-values.test.sh`, which
  reads the configured parser values out of the `.knobs` lines, and
  `gate-sdk/gate-tests/run-arm-contract.test.sh` and `enforcement-map.test.sh`. The first of those two
  needs a bridged knob a scratch config can make unresolvable, and after this cut gate-sdk is the one
  bridged kit left, so it copies `check-commit-subject.gate` and gives `GATE_SDK_COMMIT_TYPES` a tab
  through a scratch `GATE_SDK_CONFIG_FILE`. The crate tests in `native/src/emit/enforcement_map.rs` write a scratch knob file where they
  set `GATE_SDK_KNOB_EVIDENCE_KIT_*`.
- **Couples.** `evidence-kit/checks/check-battery-roster.gate` drops `scripts/evidence-config.sh`, which
  the derived knob-file couple replaces, and `gate-sdk/lib/gate.sh`, whose `gate_fixture_suites` it
  coupled to, for `native/src/registry.rs`.
- Regenerate the pre-commit hook, whose baked argv loses every `EVIDENCE_KIT_` assignment, and the graph,
  the kit SPEC mirrors and the roster readers.

### (7) guard-kit's rule content: the guard stays the one shell hook {design-bearing}

gate-sdk/SPEC.md §The knob file's bullet *guard-kit's rule content … Its cut rules whether the rules
become data or the guard stays the one shell hook* is ruled: **the guard stays the one shell hook, and
its rules stay bash.** The rules are logic, a context-aware skeleton walk, dataflow across pipes and
redirects, and a subprocess test against git. No data grammar short of a language expresses them. The
consumer's own rules live in its copy of `templates/bash-guard.sh`, composed against the library, which
is the extension point guard-kit/SPEC.md §Consumer rules rules durable.

**Of `lib/guard.sh`'s two `no-port` grounds, the first dissolves and the second holds.** The library
stops resolving knobs, so it is no longer the bridge's sole resolver for `GUARD_KIT_*`. It is still the
API a consumer's rules are written against, so it stays `# no-port:` on that ground alone.

guard-kit/SPEC.md §The guard framework (`lib/guard.sh`), a new paragraph after the primitives list.
**Not yet applied:**

> **The library reads its knobs from the binary, once per sourcing.** Its head calls `gate_knob_values`
> (gate-sdk/SPEC.md §lib/gate.sh) for every `GUARD_KIT_*` knob its rules read, and assigns each value to
> the shell variable of the same name: a scalar as a string, an indexed knob as an array, a keyed knob
> as an associative array. The rules read those variables exactly as they read the sourced config, so a
> test that reassigns one after sourcing still steers the rule. The crate's table is the knobs' one
> producer, and the library holds no default.
>
> **The load fails in one of two ways, and the hook answers each differently.** When the binary cannot
> be reached, the knob read cannot run at all, and the hook emits a `guard_advise` naming the build
> command and exits 0. An advise carries no permission decision, so every command still takes the
> harness's own permission path. Only the steering is lost, and every call says so. A block there would
> refuse the very command that builds the binary. When the binary runs and refuses the config, with a
> malformed knob file or a set-but-missing `GUARD_KIT_KNOB_FILE`, the hook blocks with the refusal's own
> text, the loud posture a set-but-missing config file always had. The file is repaired with the Edit
> tool, which the guard does not intercept.

`GUARD_KIT_LIB` is not a knob and gets no row. It names the library before the library exists, so it
stays a head-of-hook locator (gate-sdk/SPEC.md §The harness-integration arm already keeps it off every
member's slice).

The load reaches `lib/gate.sh` beside the vendor root `_guard_front_end` already derives from
`GUARD_KIT_LIB`. Until SPEC-bridge-retirement.md lands, `gate_knob_values` resolves the load's bridged
inputs, `GATE_SDK_WORKFLOW_DIR` for the two log paths, through the bridge. Build measures the hook's
per-call time before and after this delta beside the 58 to 63 ms baseline above, and records both
figures in guard-kit/SPEC.md §Testing. The bridge's retirement is what removes the bridged read from
that path.

`native/src/emit/run_guard_tests.rs` exports an absolute `GATE_SDK_NATIVE_BIN` into each case's
environment, beside `GUARD_KIT_LIB` and `GUARD_KIT_LOG`, because a case runs in a sandbox where the
repo-relative default names nothing.

**Refused.** Porting the guard to a `--hook bash-guard` member. It deletes the consumer-rules extension
point, and a cut narrows the port, never an extension point. Also refused: a bash reader of the knob
file inside `lib/guard.sh`. The library would then hold every default a second time beside the crate's
table, the second producer. And a load through `bash gate-sdk/bin/run-gates.sh --emit knob-values`: that
front-end refuses outside a checkout, and the decision-table harness runs every case in a sandbox.

### (8) guard-kit goes static {mechanical}

guard-kit gains `native/src/knobs/guard_kit.rs` and joins `STATIC_KITS`. Every row transcribes from
`guard-kit/lib/guard.sh` lines 23 to 39 as they stand at this amendment's commit:

- **Derived scalars:** `GUARD_KIT_LOG` and `GUARD_KIT_WAKEUP_LOG`, from `GATE_SDK_WORKFLOW_DIR`.
- **Scalars:** `GUARD_KIT_SETTINGS` and `GUARD_KIT_SETTINGS_LOCAL`.
- **Indexed:** `GUARD_KIT_BREADTH_PROBES` (empty), `GUARD_KIT_RO_SCRIPTS`, `GUARD_KIT_SCRATCH_DIRS`,
  `GUARD_KIT_RO_BINS`, `GUARD_KIT_APPEND_BINS`, `GUARD_KIT_SEARCH_TOOLS` and
  `GUARD_KIT_SCRIPT_INTERPRETERS`, each with its library default.
- **Keyed:** `GUARD_KIT_BREADTH_DECLARED` and `GUARD_KIT_RO_FORMS` (both empty).

Every live consumer value is expressible. A breadth declaration's key is a permission rule, such as
`Bash(git worktree *)`, carrying no `]`, `=` or tab, and its reason carries no tab.

- Lines 5 to 39 of `lib/guard.sh`, the config source and the defaults, are replaced by delta 7's load.
- `git mv scripts/guard-config.sh scripts/guard-config.knobs`, rewritten: eleven
  `GUARD_KIT_BREADTH_PROBES[] =` lines, five `GUARD_KIT_BREADTH_DECLARED[<rule>] =` lines, and
  `GUARD_KIT_SEARCH_TOOLS =`, empty. `templates/guard-config.sh` becomes `templates/guard-config.knobs`,
  its examples as commented `NAME[] =` and `NAME[key] =` lines.
- **Tests.** `guard-kit/gate-tests/guard-config-knobs.test.sh` writes its seven configs as knob files
  under `GUARD_KIT_KNOB_FILE`. `compare-settings-allow.test.sh` writes its probes and declarations as knob
  lines. `scan-prompts.test.sh` keeps its scalar exports. `git-mutation-under-producer.test.sh` keeps its
  post-source array assignment, which delta 7 keeps working. `guard-kit/smoke/install.sh` copies
  `templates/guard-config.knobs`. `guard-kit/gate-tests/guard-lib-parity.test.sh` sources the library under
  the hermetic harness, whose `GUARD_KIT_KNOB_FILE` pin now reaches it.
- The four crate readers, `compare_settings_allow.rs`, `drift_report.rs`, `scan_prompts.rs` and
  `hook/wakeup.rs`, read through `walk::knob_*` unchanged, which routes a static name to the table.
- Regenerate the graph and the kit SPEC mirrors.

### (9) Two steers for the unsteered compositions {design-bearing}

guard-kit/SPEC.md §The generic ruleset gains two rules after rule 24. Fall-through logging, rule 25
today, becomes rule 27, and its two citations (`guard-kit/SPEC.md` §scan-prompts and the
`guard-kit/guard-tests/cases.tsv` comment) follow. **Not yet applied:**

> 25. **An emitter write the harness cannot grant** — blocked, with a steer per arm. Both arms read a
>     statement whose leading command is a `GUARD_KIT_APPEND_BINS` member writing through `>>` or `>`,
>     the shape rule 17 grants, and declare rule 17's classes.
>     - **(a) Compounded.** The command holds more than one statement, and one of them, issued alone,
>       satisfies every clause of rule 17. The steer is to issue that write as its own call, which rule 17
>       grants with no permission decision, and the rest as a separate call.
>     - **(b) A target git does not ignore.** The statement is the whole command, and a target fails rule
>       17's `git check-ignore` test. The steer is the harness's Write or Edit tool for a file, or the
>       capture arm that owns the surface when the target is one: the tool is reviewable where a redirect
>       is not, and the arm keeps the surface's grammar.
>     **What was measured.** The friction log this rule was cut from, read at spec, holds twenty
>     resume-journal appends, every one to a path under `.tmp/`. Sixteen are followed by a further
>     statement, one is preceded by one, and the other three run past the log's 500-character truncation.
>     So arm (a) is the measured class. Arm (b) is the entry's stated better form for the tracked
>     direction, which that log holds no instance of, and it is kept because the steer is the READ
>     direction's mirror (rule 10). The destination test both arms need is rule 17's, so no consumer
>     roster is owed.
> 26. **A `bash -c` or `sh -c` wrapper** — blocked, with the steer to run the payload as the command
>     itself, or, for a body that needs a shell of its own, to write it to a scratch script and run that
>     through the `--scratch-run` arm. Fires when a statement's command word is `bash` or `sh` and its
>     first option is `-c`. The wrapper puts the whole payload inside one quoted argument, which the
>     skeleton strips, so no allowlist entry and no rule above can see the command inside it. That makes
>     the wrapper a steering target on its own terms, whatever it wraps. Declares `sq dq hd`.
>     **What it does not reach.** A wrapper behind another command word, `xargs bash -c` or `timeout 5
>     bash -c`, is out of scope: the leading word decides which rule reads the call, and each of those
>     leads is its own rule's subject. A `bash <script>` naming a scratch path is rule 23's.
>
> Both rules block rather than advise, on rule 20's reasoning: the call would take a permission decision
> anyway, and a block turns that decision into a durable steer at no extra cost. Both sit at the tail,
> after rule 24, because neither grants anything and a command reaching them has been declined by every
> grant above.

`lib/guard.sh` gains `guard_rule_emitter_write` and `guard_rule_shell_wrapper`, called from
`guard_generic_rules` after rule 24. `guard-kit/guard-tests/cases.tsv` gains rows: a journal append
followed by a `git` line blocks; the same append alone still allows; `cat >> tracked.md` alone, which
falls through today, blocks; `bash gate-sdk/bin/run-gates.sh --emit trajectory > docs/x.md`, led by no
roster emitter, does not fire; `bash -c 'git status'` blocks; `xargs bash -c 'x'` does
not fire; `bash gate-sdk/bin/run-gates.sh --scratch-run .tmp/x.sh` does not fire. The
`append-and-wrapper-shapes-unsteered` entry's measured `bash -c` class, 20 calls, predates the friction
log's reset at this iteration's scope, so its shapes are not re-read here. The `xargs` and `timeout` rows
pin the stated boundary instead.

The existing row `fallthrough cat >> tracked.md <<'EOF'…` becomes a `block` row under arm (b).

**Refused.** Naming the `--emit kfric`, `file-gap` and `file-survey` arms, or the paths they own, in arm
(b)'s steer. It would put lifecycle-kit's and drift-kit's surfaces inside guard-kit, a destination
roster a consumer would then have to keep in step, where the generic steer lets the agent's own
instructions name the arm. Also refused: an advise on the wrapper. It would fall through to the same
prompt and teach nothing durable.

### (10) The kits' own SPEC sections {design-bearing}

**The evidence-kit bullets are applied at build, second batch**; the guard-kit bullets and the
`--emit-scan-prompts` bullet are the third batch's.

- **evidence-kit/SPEC.md §Layout and configuration** states the kit static, as site-kit/SPEC.md does:
  the knob file, `EVIDENCE_KIT_KNOB_FILE`, the `.local.knobs` overlay and a pointer to gate-sdk/SPEC.md
  §The knob file. The *permanently shell* sentence goes. The `EVIDENCE_KIT_RUN_<suite>` and
  `_PARSER_<suite>` bullets state the declared families and the derived run members (delta 3). The
  *resolved inside this kit's already-sourced subshell* clause goes with the bridge's reading of it. A
  new `EVIDENCE_KIT_FIXTURE_SUITES` bullet states the derivation and the reference idiom (deltas 2 and 5).
  `EVIDENCE_KIT_RUNNER_DOC`'s *the loader fills no default for it* sentence becomes *its default is
  `README.md`*.
- **evidence-kit/SPEC.md §lib/evidence.sh becomes §The evidence adapters**, describing
  `native/src/evidence.rs` as the sole holder. The parser-adapter rules, the `libtest` reservation, the
  consumer-command contract and the parity-shape record stay. The *permanently shell* paragraph, the
  config-loader sentence and the adapter-twin history paragraphs are deleted, because their subjects
  are. The `// spec:` citations to the old heading follow the rename.
- **evidence-kit/SPEC.md §bin/run-validate.sh's** declared-name count and §Evidence manifest's contract
  token sentence follow deltas 5 and 6.
- **guard-kit/SPEC.md §Layout and configuration** states the kit static, copying
  `templates/guard-config.knobs`, and the *permanently shell* sentence about the template and its copy
  goes. `GUARD_KIT_LOG` and `GUARD_KIT_WAKEUP_LOG` state their derived defaults.
  `GUARD_KIT_BREADTH_DECLARED` and `GUARD_KIT_RO_FORMS` show the keyed line form, and their *associative
  rather than a delimited indexed array* ground stays, restated for the keyed line. The
  `GUARD_KIT_RO_FORMS` bullet's *No bridged arm declares it, so no crate reader exists and no second
  resolver is owed* sentence goes, because the table is its one producer.
- **guard-kit/SPEC.md §The guard framework (`lib/guard.sh`)** states the one remaining `no-port` ground
  and delta 7's load, and its two-ground paragraph loses ground (1). §Consumer rules is unchanged.
  §Testing gains delta 7's latency figures and delta 9's rows.
- **gate-sdk/SPEC.md §The non-gate arm**, the `--emit-scan-prompts` paragraph, which calls that member's
  whole configuration *a permanently-shell library's*: the three knobs are guard-kit's table rows, and
  the forced-family argument is restated on the table.

### (11) gate-sdk's standing rules and the release declarations {mechanical}

**The evidence-kit halves are applied at build, second batch**: the `evidence` stem, the generated
family and delta 4's exception in §The knob file, §lib/gate.sh's two bullets, the live declarers of
both port dispositions re-pointed at guard-kit's template and library, and evidence-kit's declarations.
The guard-kit halves are the third batch's.

- **gate-sdk/SPEC.md §The knob file:** the `<stem>` examples gain `evidence` and `guard`. The
  unruled-shapes list loses the generated family (delta 3) and guard-kit's rule content (delta 7), and
  keeps *gate-sdk migrates last*. The reference refusal list gains delta 4's exception.
- **gate-sdk/SPEC.md §lib/gate.sh**, the prefix-family bullet: its live instance and its
  *loop-declared variables* paragraph become the static family's (delta 3). Its `scripts/evidence-config.sh`
  `while`-loop sentence goes with the loop. The bullet `gate_fixture_suites is the single source …` names
  `registry::fixture_suites()` and the arm (delta 2).
- **gate-sdk/SPEC.md §The config-seam port disposition:** the class has no shell member left, so the
  *live declarer* sentence citing `evidence-kit/templates/evidence-config.sh` states that the class is
  empty and the ground stands for the knob files. **§The kit-library port disposition:** the member-level
  condition has fired for evidence-kit, and `guard-kit/lib/guard.sh`'s worked-instance paragraph keeps
  only its second ground.
- **gate-sdk/SPEC.md §upgrade-smoke** gains delta 1 (applied at build, first batch).
- **`.workflow/release-declarations.md`.** **Evidence-kit's halves applied at build, second batch;
  guard-kit's not yet applied:**
  - Renamed knobs: `EVIDENCE_KIT_CONFIG_FILE` → `EVIDENCE_KIT_KNOB_FILE`, `GUARD_KIT_CONFIG_FILE` →
    `GUARD_KIT_KNOB_FILE`.
  - Behavior changes: **`<gates-dir>/evidence-config.sh` and `<gates-dir>/guard-config.sh`** are replaced
    by `.knobs` files, with defaults printed by `run-gates.sh --emit knob-roster`. A left-behind shell
    config is refused at exit 2, **including the copy an earlier `init` seeded that you never edited**:
    delete it, or rewrite what you set into the `.knobs` file `init` now seeds. An exported scalar knob
    of these kits now outranks the file. A config that built `EVIDENCE_KIT_SUITES` and
    `EVIDENCE_KIT_RUN_<suite>` in a loop over `gate_fixture_suites` writes
    `EVIDENCE_KIT_SUITES[] <- EVIDENCE_KIT_FIXTURE_SUITES` instead, and the run members for those suites
    are derived. **`gate_fixture_suites` is removed**: a workflow copied from
    `gate-sdk/templates/gates-workflow.yml` loops over `run-gates.sh --emit fixture-suites` instead, as the
    template now shows. guard-kit's hook reads its knobs through the gate binary, so a hook running where
    the binary is absent steers nothing and says so on every call. **Two new guard rules block**: an
    append compounded with a second statement, and a `bash -c` or `sh -c` wrapper.
  - Tightened gates: every gate reading an `EVIDENCE_KIT_` or `GUARD_KIT_` knob, the set build derives
    from `checkwright-gates --knob-files` and a per-gate refusal probe over a scratch tree holding a
    left-behind shell config, cut 3's method.

## Producers and consumers

- **`registry::fixture_suites()` and `--emit fixture-suites`** (new interface). *Producer:* the crate
  derivation over the kit roots and the gates directory. *Consumers:* `EVIDENCE_KIT_FIXTURE_SUITES`' and
  the `EVIDENCE_KIT_RUN_` family's derivations, and the two workflows' fixture step, which reads all three
  fields at the loop: the suite as the banner, the tests and checks directories as the arm's operands.
  *Enabling config:* none, and a tree with no `gate-tests/` directory yields no rows.
- **`Family` and `Kit.families`** (new state). *Producer:* the kit tables. *Readers:* `knobs::layer`,
  which admits a member; `knobs::family`, which enumerates members for `walk::knob_prefix`; and
  `knobs::bridged`, which reads the family's declared inputs. The derivation field is read at
  resolution, the inputs field at `--knobs`.
- **`Row.referents`** (new field). *Readers:* `refuse_referent` at parse, and `bridged_inputs` at
  `--knobs`. *Enabling config:* this repo's `scripts/evidence-config.knobs` reference line.
- **The evidence-kit and guard-kit tables and validators** (new state). *Producer:*
  `native/src/knobs/{evidence,guard}_kit.rs`. *Consumers:* `knobs::resolve` for every read of the two
  prefixes, the two roster arms, the undeclared-name refusal and each validator at first resolution.
- **`lib/guard.sh`'s knob load** (changed interface). *Producer:* `gate_knob_values`, over the binary's
  `--emit knob-values`. *Consumers:* the rule functions that read the thirteen shell variables, the
  consumer's own rules in its hook copy, and the tests that source the library. Each value line's name,
  shape and element are read by the assignment, the shape choosing string, array or map.
- **The upgrade smoke's seeding and retirement steps** (new behaviour). *Producer:* `upgrade_smoke.rs`,
  through `recipe::config_seam_plan`. *Consumers:* the FROM baseline battery, phase A's regen, which
  needs the retired files gone, then phase B's red set against TO's Tightened-gates declaration.
- **Rules 25 and 26** (new behaviour). *Producer:* `lib/guard.sh`. *Consumers:* the harness, which
  shows the block, and the decision-table runner, which reads the exit status.
- **Red conditions (point 5).** Deltas 6 and 8 narrow corpora by deleting a library and replacing two
  shell configs and two templates. Delta 2 deletes a shell function.
  - *`check-docs-cmd` assertion B's known-knob set* reds on a documented name absent from the set. The
    static roster covers every row, and the two families' `*` spellings are documented names that the
    set must hold, so build runs the gate rather than reasoning about it.
  - *`check-knob-default-coupling`* reds on a SPEC default disagreeing with its source. The migrated
    defaults move to the roster idiom, and a derived default classifies as computed. Build runs it.
  - *§port-blockers' `--tree` counts and every measured claim over them* read exact counts. One library,
    two consumer configs and one template leave the scanned shell set, and `lib/guard.sh` stays declared.
    `check-measured-claim` re-runs its oracles, and build runs the battery.
  - *`check-comment-tier`* reds on an ungoverned comment. `CANON_KIT_COMMENT_SURFACE` already globs
    `*.knobs`, so the comments move under it with their directives.
  - *`check-graph` assertion D* reds on a stale hook, and deltas 5 and 6 change the baked argv of every
    evidence-kit member. Delta 6 regenerates.
  - *`check-reads-couples`* compares declarations with couples. `check-battery-roster` loses two hand
    couples and gains the derived knob file and `native/src/registry.rs`. Build runs it.
  - *`check-template-copy-parity`* excludes `*-config.knobs` by suffix already, so the net finding set is
    unchanged by inspection. Build runs it.
  - *The decision table* reds on a row whose decision moved. Delta 9's rules are new blocks, and a
    compounded journal append in an existing row would change from fall-through to block. Build runs
    `--run-guard-tests` and reads every changed row.
  - *`evidence-kit/smoke/install.sh` and the upgrade suite* red when a run misses a suite. Delta 5's
    reference keeps the roster. Delta 1's seeding adds only absent files, and its retirement step
    deletes evidence-kit's `evidence-config.sh` once delta 6 replaces the template, so build re-runs the
    upgrade suite after delta 6 and declares any gate that reds without the smoke-set values.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The knob file (deltas 3, 4 and 11); §lib/gate.sh, the prefix-family bullet and
  the fixture-suite bullet (deltas 2, 3 and 11); §upgrade-smoke (delta 1); §The config-seam port
  disposition and §The kit-library port disposition (delta 11); §The non-gate arm, the
  `--emit-scan-prompts` paragraph (delta 10).
- `evidence-kit/SPEC.md` — §Layout and configuration, §lib/evidence.sh renamed, §bin/run-validate.sh,
  §Evidence manifest (deltas 2, 3, 5, 6 and 10).
- `guard-kit/SPEC.md` — §The guard framework, §The generic ruleset, §scan-prompts' rule citation,
  §Layout and configuration, §Testing (deltas 7, 8, 9 and 10).
- `evidence-kit/README.md` — config step (delta 6).
- `guard-kit/README.md` — config steps (delta 8).
- `installer/README.md` — the `scripts/evidence-config.sh` mention (delta 6).
- `guard-kit/templates/close-triage.md` — the `guard-config.sh` mention (delta 8).
- `native/src/knobs/mod.rs` — `STATIC_KITS`, `Family`, `Row.referents` (deltas 3, 4, 5 and 8).
- `native/src/knobs/evidence_kit.rs` — new table and validator (delta 5).
- `native/src/knobs/guard_kit.rs` — new table (delta 8).
- `native/src/walk.rs` — `knob_prefix` over a static family (delta 3).
- `native/src/registry.rs` — `fixture_suites` (delta 2).
- `native/src/emit/mod.rs` — the `--emit fixture-suites` row (delta 2).
- `native/src/emit/run_validate.rs` — the unread `EVIDENCE_KIT_STATE_FILE` declaration (delta 5).
- `native/src/emit/upgrade_smoke.rs` — the seeding and retirement steps (delta 1).
- `docs/install.md` — §The upgrade contract, the phase-A retirement step (delta 1).
- `lifecycle-kit/templates/upgrade.md` — step 1 names the retirement step (delta 1).
- `native/src/emit/run_guard_tests.rs` — the absolute binary export (delta 7).
- `native/src/emit/enforcement_map.rs` — test knob writers (delta 6).
- `native/src/evidence.rs` — the parity test deleted, the section citations (deltas 6 and 10).
- `native/src/emit/parse_gates_log.rs` — the section citation (delta 10).
- `native/src/gates/evidence_manifest.rs` — the section citation (delta 10).
- `evidence-kit/lib/evidence.sh` — deleted (delta 6).
- `evidence-kit/templates/evidence-config.sh` — replaced by the `.knobs` template (delta 6).
- `evidence-kit/checks/check-battery-roster.gate` — couples (delta 6).
- `evidence-kit/smoke/install.sh` — config writer (delta 6).
- `evidence-kit/gate-tests/check-battery-roster.test.sh` — config writer (delta 6).
- `evidence-kit/gate-tests/check-battery-roster/bad/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/check-battery-roster/good/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/check-evidence-baseline.test.sh` — config writer (delta 6).
- `evidence-kit/gate-tests/check-evidence-baseline/bad/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/check-evidence-baseline/good/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/check-evidence-manifest.test.sh` — config writer (delta 6).
- `evidence-kit/gate-tests/check-evidence-manifest/bad/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/check-evidence-manifest/good/scripts/evidence-config.sh` — moved to `.knobs` (delta 6).
- `evidence-kit/gate-tests/diff-baseline-status.test.sh` — config pin (delta 6).
- `evidence-kit/gate-tests/evidence-lib.test.sh` — config pin, library citation (delta 6).
- `evidence-kit/gate-tests/producer-lock.test.sh` — config writer (delta 6).
- `scripts/evidence-config.sh` — moved to `.knobs`, with the reference (deltas 4 and 6).
- `scripts/gate-tests/evidence-parser-values.test.sh` — reads the `.knobs` lines (delta 6).
- `gate-sdk/gate-tests/enforcement-map.test.sh` — config pin (delta 6).
- `gate-sdk/gate-tests/run-arm-contract.test.sh` — the bridged example (delta 6).
- `gate-sdk/lib/gate.sh` — `gate_fixture_suites` deleted (delta 2).
- `gate-sdk/templates/gates-workflow.yml` — the arm loop (delta 2).
- `.github/workflows/gates.yml` — the arm loop (delta 2).
- `guard-kit/lib/guard.sh` — the load, rules 25 and 26 (deltas 7, 8 and 9).
- `guard-kit/templates/guard-config.sh` — replaced by the `.knobs` template (delta 8).
- `guard-kit/guard-tests/cases.tsv` — new rows, the renumbered comment (delta 9).
- `guard-kit/smoke/install.sh` — template copy (delta 8).
- `guard-kit/gate-tests/guard-config-knobs.test.sh` — knob-file writers (delta 8).
- `guard-kit/gate-tests/compare-settings-allow.test.sh` — knob-file writers (delta 8).
- `scripts/guard-config.sh` — moved to `.knobs` (delta 8).
- `scripts/git-hooks/pre-commit` — regenerated (deltas 5 and 6).
- `.workflow/release-declarations.md` — Renamed knobs, Behavior changes, Tightened gates (delta 11).
- `TASK-QUEUE.md` — the paired entries' landing moves and the bridge entry's figure (deltas 1, 9 and 11).
- `docs/check-graph.html` — generated artifact, regenerated (delta 6).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/evidence-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/evidence-kit/README.md` — generated mirror, regenerated (delta 6).
- `docs/guard-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/guard-kit/README.md` — generated mirror, regenerated (delta 8).
<!-- update-target-exempt: the survey record is boundary-truncated at the next scope, and its finding quotes the pre-cut file names as the measurement it was -->
- `.workflow/survey-record.md` — unchanged.
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the adapter it shipped stays -->
- `docs/posts/2026-07-18-checkwright-v0-6-0.md` — unchanged.

## Retired spellings

- `lib/evidence.sh` — deleted (delta 6).
- `evidence-config.sh` — replaced by `evidence-config.knobs` (delta 6).
- `EVIDENCE_KIT_CONFIG_FILE` — renamed `EVIDENCE_KIT_KNOB_FILE` (deltas 6 and 11).
- `ek_suite_cmd` — the shell adapter, deleted with its library (delta 6).
- `ek_queue_iteration` — the shell adapter, deleted with its library (delta 6).
- `ek_state_stage` — the shell adapter, deleted with its library (delta 6).
- `ek_run_key` — the shell adapter, deleted with its library (delta 6).
- `ek_data_lines` — the shell adapter, deleted with its library (delta 6).
- `EVIDENCE_MANIFEST_CONTRACT` — the shell twin of `evidence::MANIFEST_CONTRACT` (delta 6).
- `gate_fixture_suites` — replaced by `--emit fixture-suites` (delta 2).
- `guard-config.sh` — replaced by `guard-config.knobs` (delta 8).
- `GUARD_KIT_CONFIG_FILE` — renamed `GUARD_KIT_KNOB_FILE` (deltas 8 and 11).

## Definition of Done

- [ ] **Causal completeness** — the family, the referent, the fixture-suite arm, the knob load's two
      failure answers, the seeding step and rules 25 and 26 each have a unit test or a decision-table row.
- [ ] **Instruction surfaces: instruction only** — the refusal messages, the templates, the steers and the
      release declaration carry the remedy, not the grounds.
- [ ] **Merged with no information lost** — each ruling lands in its SPEC section; the refused
      alternatives stay in this file's history.
- [ ] **Amendment deleted**, and `config-seam-static-format` keeps SPEC-bridge-retirement.md's ref; the
      other two paired entries move to Done.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any reader delta 6's roster missed, and `EVIDENCE_KIT_PRE_HOOK`'s absent test
      writer if delta 5 does not add one.
