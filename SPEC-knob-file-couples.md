# SPEC amendment: knob-file-couples

Queue entry: `static-knob-file-couple-underived`. It rides unit set `config-seam-third-cut` beside
`config-seam-static-format`'s cut 3 (operator direction, 2026-09-14, lead-relayed), and it lands
**before** that cut's deltas: once a static kit's knob file is derived into its readers' triggers, the
cut's four kits need no hand couple on any descriptor. SPEC-knob-files-cut-3.md extends the
derivation with the one reach its reference form adds.

The entry asks where a derivation from the registry's static-knob declarations stops, and whether an
assertion holds the rest. This amendment rules that the derivation reaches every shape the entry
names but one, that the remaining shape is a consumer command's own reads and has no kit-side
oracle, and that the derivation replaces the hand couples rather than being asserted against them.

## The seam

- **Kit mechanism:** the derivation (`registry::knob_files`), its `--knob-files` arm, and the four
  trigger readers that append it.
- **Consumer config:** none new. The derived path is spelled from the consumer's gates directory, so
  the hand couples it replaces, which spelled this repo's `scripts/` inside kit-shipped descriptors,
  leave the kits.
- **Private rule content:** none in reach.

## The measured reader set

Every registered gate was run alone (`run-gates.sh --only <gate>`) once per static kit, with that
kit's `<KIT>_KNOB_FILE` pointed at a file holding one malformed line, at `c8059acc`. A gate that
exits non-zero reads that kit's knob file. The set is this amendment's acceptance oracle (delta 4):

- **queue-kit (15):** `check-queue-sections`, `check-queue-hygiene`, `check-queue-wrap`,
  `check-tag-lead-line`, `check-queue-entry-budget`, `check-deferred-board-tags`, `check-task-names`,
  `check-task-conservation`, `check-queue-prose-precondition`, `check-queue-slug-liveness`,
  `check-roadmap-fresh`, `check-prose-enum`, `check-graph`, `check-reads-couples`,
  `check-gate-substrate-parity`.
- **lifecycle-kit (16):** `check-lifecycle-registration`, `check-trajectory-fresh`,
  `check-stage-evidence`, `check-stage-entry`, `check-stage-skill-coverage`, `check-skill-binding`,
  `check-shim-restatement`, `check-lesson-disposition`, `check-merge-attrs`, `check-close-surfaces`,
  `check-survey-record`, `check-scratch-citation`, `check-gap-inbox-neutrality`, `check-graph`,
  `check-reads-couples`, `check-gate-substrate-parity`.
- **site-kit (2):** `check-docs-cname-parity`, `check-docs-render-fidelity`.
- **doctrine-kit (1):** `check-doctrine-registration`.

A whole-battery run cannot take this measurement: the battery front-end resolves the registry's knob
union before any member runs, so one malformed file fails the run at resolution and names no reader.

The entry's three uncovered shapes resolve against that set as follows. The three meta-gates read
queue-kit's and lifecycle-kit's files, and no other kit's, because those are the kits a `knob:` token
in the descriptor corpus names. `check-prose-enum` reads queue-kit's file through the command its
canon-kit knob names. Beside those four, one measured reader's read is reached by no declaration of
its own: `check-trajectory-fresh` reads lifecycle-kit's file through drift-kit's still-bridged config,
which copies the stage roster through the transitional `gate_static_knob`. `run-gates --for
scripts/lifecycle-config.knobs` does not select it at `c8059acc`, so that miss is live today. It
closes at cut 3, where the copy becomes the knob-file reference form and delta 1's reach extends to it.

## What changes

### (1) A member's knob files are derived from its declaration {design-bearing}

gate-sdk/SPEC.md §The `# graph:` manifest, after the paragraph that closes on *`couples=` must cover
every path the gate reads at runtime, never a subset*. **Not yet applied:**

> **A static kit's knob file is a derived couple, never a hand-written one.** A member that reads a
> static knob reads its kit's knob file, so the couple is a function of the member's registry
> declaration, and `registry::knob_files` computes it. The set holds
> `<gates-dir>/<stem>-config.knobs` for every static kit the declaration reaches, in the static-kit
> table's order, where a declaration reaches a kit through:
>
> - **a declared name the kit owns**, a `<KIT>_…*` family name included;
> - **the couples-knob sentinel**, which reaches each static kit owning a name a `knob:` token in the
>   descriptor corpus carries. That is the sentinel's static half, computed from the corpus exactly as
>   its bridged half is (§lib/gate.sh);
> - **a derived default's declared input** that another static kit owns, transitively.
>
> The gates directory is the locator the static reader itself uses, env-or-default (§The knob file).
> The set names the tracked file at its default location and nothing else. The local overlay is
> gitignored and never staged, so it fires no trigger. A tracked file relocated through
> `<KIT>_KNOB_FILE` is a per-invocation path that a tracked hook cannot bake, so a consumer who
> relocates one couples it by hand.
>
> **The derivation stops at a command.** A knob whose value is a command hands its reads to the
> program it names, and the kit cannot see them. A member spawning a consumer's command couples the
> command knob's own kit file, which the rules above derive, and the program's reads are the
> consumer's to couple.
>
> A `mode=staged` member that reaches a static kit is refused at exit 2 by every reader of the
> derivation. The staged hook passes matched paths to the gate as arguments, so a derived knob file
> would reach the gate as a file to scan. No staged member reads a static knob.

Mechanism: `registry::knob_files(member) -> Result<Vec<String>, String>` over
`gates::declared(member)`, `knobs::owner`, the rows' `inputs`, and a static counterpart of
`registry::couples_knob_names` for the sentinel. A unit test runs it for every registry member and
holds two things: each member declaring a static name gets that kit's file, and each sentinel member
gets exactly the kits the corpus's static tokens name.

### (2) The trigger readers append the derived set {design-bearing}

A member's trigger is its expanded `trigger=` field, or its expanded `couples=` when `trigger=` is
absent, followed by `registry::knob_files(member)`. A `trigger=*` member is unchanged. Four readers
take a member's trigger or its coupling edges, and each appends the set:

- **`run-gates --for`** (`native/src/runner.rs`, the selection loop).
- **`check-graph`**'s manifest loop (`native/src/gates/graph.rs`). It appends to both the expanded
  `couples=` and the expanded `trigger=`, so assertion B's couples-within-trigger parity is unchanged.
  The append follows the `GRAPH_VOCAB` surface check, which reads the authored field alone: a derived
  path is no surface a consumer wrote, and a consumer vocabulary need not list it.
- **The graph emitter** (`native/src/emit/graph.rs`), so the published coupling graph draws the edge
  a derived couple creates.
- **Hook generation.** `bin/gen-pre-commit.sh` stays shell, so it reads the set from the binary's new
  top-level arm, once per process, and `emit_block` appends a member's paths after
  `gate_expand_couples_var`. Assertion D of `check-graph` regenerates the hook, so the hook and
  `--for` cannot diverge without a red.

gate-sdk/SPEC.md §lib/gate.sh, beside `--knobs`. **Not yet applied:**

> `--knob-files [<gate-name>...]` prints one `<gate-name>`⇥`<path>` line per derived knob file, for
> the named members or, with no argument, for every registry member. A member deriving none prints
> nothing. A name the registry does not carry is exit 2, and so is a `mode=staged` member that reaches
> a static kit. It reads no knob and resolves no bridge.

`.workflow/release-declarations.md`, Behavior changes. **Not yet applied:**

> - **Generated pre-commit hook** — a gate reading a static kit's knob file now fires on an edit to
>   `<gates-dir>/<stem>-config.knobs` with no couple on its descriptor; regenerate your hooks. A
>   knob file you relocated with `<KIT>_KNOB_FILE` fires nothing unless your own descriptor couples it.

Three readers of `registry::expand_couples` take the field for another purpose and stay unchanged:

- **`check-reads-couples`** asks whether a member's couples cover its walk roots. A knob file is a
  named-file read, which §check-reads-couples rules outside the walk class.
- **`check-gate-substrate-parity` assertion C** asks whether a member's couples cover a gate
  declaration path. A knob file never is one.
- **`port-blockers`** prints the descriptor's own expanded field as evidence.

### (3) The hand couples leave the descriptors {mechanical}

Every `couples=` or `trigger=` token naming `scripts/<stem>-config.knobs` or
`scripts/*-config.knobs` is deleted wherever delta 1 derives the same file for that member. Build
takes the roster from `git grep -n 'config.knobs' -- '*.gate'` and holds each deletion to delta 4's
oracle. At this amendment's commit that is every such token but one.

- **Kept:** `canon-kit/checks/check-prose-enum.gate`'s `scripts/queue-config.knobs`. It is the command
  residue delta 1 names. This repo's `CANON_KIT_ENUM_SETS_CMD` runs `--emit enum-sets`, which reads
  queue-kit's lesson tags, and no declaration of that member reaches queue-kit.
- `check-graph`'s `scripts/*-config.knobs` goes: its sentinel derives the two kits whose values the
  hook's `knob:` expansions read, which is the edit that glob was written to catch.
- The `native/src/knobs/<kit>.rs` couples stay. They are the source-module axis, filed as
  `in-crate-module-coupling-derivation`.
- Regenerate the pre-commit hook and the published graph.

### (4) The measured reader set is the acceptance oracle {mechanical}

Before and after delta 3, build runs `run-gates.sh --for scripts/<stem>-config.knobs` for each of the
four static kits, with `GATE_SDK_VERBOSE` set so the per-gate banners name the selection. Each
post-delta selection must contain that kit's set under *The measured reader set* above, except
`check-trajectory-fresh` for lifecycle-kit, which SPEC-knob-files-cut-3.md's oracle holds. A gate
selected before and not after is a lost trigger and a red. A gate selected after that the set does not
contain is an over-trigger: broad and `trigger=*` members are selected for any path, and build reports
them without failing on them.

## Producers and consumers

- **`registry::knob_files`** (new interface). *Producer:* the crate, from registry data a member
  cannot compile without, the static-kit table and the descriptor corpus. *Consumers:* `--for`,
  `check-graph`, the graph emitter and `--knob-files`. Enabling config: none, since every registered
  member declaring a static knob fires it under the default layout.
- **`--knob-files`** (new arm). *Producer:* `native/src/main.rs`, as a top-level flag beside `--knobs`.
  *Consumer:* `bin/gen-pre-commit.sh`'s `emit_block`. Column one keys the member and column two is the
  appended path; both are read.
- **The derived trigger** (changed state). *Consumers:* the generated pre-commit hook, `--for`
  selection, `check-graph` assertions B and D, and `docs/check-graph.html`.
- **Red conditions (point 5).** Delta 3 narrows every hand-coupled descriptor's field, and delta 2
  widens four readers.
  - *The hook and `--for` selection* go red by omission: a narrowed trigger that loses a reader skips
    a red gate. That is not monotone, so it is not cleared by inspection. Delta 4 measures it.
  - *`check-graph` assertion D* reds on a stale hook. Deltas 2 and 3 both change the hook, and delta 3
    regenerates it.
  - *`check-graph`'s vocabulary check* reds on a couples surface outside `GRAPH_VOCAB`. Delta 3
    removes authored surfaces, and the check reads only authored ones, so it can only lose findings.
    This repo sets no vocabulary, so the check is inert here either way.
  - *`check-graph`'s `knob:` admissibility* reds on an undeclared token. Delta 3 removes no `knob:`
    token, and delta 2 adds none.
  - *`check-reads-couples`* reds on a walk root outside couples. It does not read the derived set,
    and no knob file is a walk root, so delta 3 cannot red it.
  - *`check-gate-fixture-coverage`* derives its set from arms and descriptors. Build runs it after the
    top-level arm lands and states whether that set takes top-level flags.
  - *`check-measured-claim`* re-runs its oracles, and a descriptor-count claim over couples would
    move. Build runs the battery rather than inspecting.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The `# graph:` manifest (delta 1); §lib/gate.sh, the `--knobs` paragraph's
  neighbour and the sentinel paragraph's static half (deltas 1 and 2); §gen-pre-commit, the trigger
  expansion (delta 2); §run-gates, `--for` selection (delta 2); §check-graph, the manifest loop and the
  emitted edges (delta 2).
- `native/src/registry.rs` — `knob_files` and the sentinel's static half (delta 1).
- `native/src/main.rs` — the `--knob-files` arm (delta 2).
- `native/src/runner.rs` — `--for` selection (delta 2).
- `native/src/gates/graph.rs` — the manifest loop (delta 2).
- `native/src/emit/graph.rs` — emitted edges (delta 2).
- `gate-sdk/bin/gen-pre-commit.sh` — `emit_block` (delta 2).
- `canon-kit/checks/check-prose-enum.gate` — unchanged; its kept couple is the residue (delta 3).
- `gate-sdk/checks/check-graph.gate` — couples (delta 3).
- `gate-sdk/checks/check-reads-couples.gate` — couples (delta 3).
- `gate-sdk/checks/check-gate-substrate-parity.gate` — couples (delta 3).
- `doctrine-kit/checks/check-doctrine-registration.gate` — couples (delta 3).
- `site-kit/checks/check-docs-render-fidelity.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-sections.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-hygiene.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-wrap.gate` — couples (delta 3).
- `queue-kit/checks/check-tag-lead-line.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-entry-budget.gate` — couples (delta 3).
- `queue-kit/checks/check-deferred-board-tags.gate` — couples (delta 3).
- `queue-kit/checks/check-task-names.gate` — couples (delta 3).
- `queue-kit/checks/check-task-conservation.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-prose-precondition.gate` — couples (delta 3).
- `queue-kit/checks/check-queue-slug-liveness.gate` — couples (delta 3).
- `queue-kit/checks/check-roadmap-fresh.gate` — couples and trigger (delta 3).
- `lifecycle-kit/checks/check-lifecycle-registration.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-stage-evidence.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-stage-entry.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-stage-skill-coverage.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-skill-binding.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-shim-restatement.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-lesson-disposition.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-merge-attrs.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-close-surfaces.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-survey-record.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-scratch-citation.gate` — couples (delta 3).
- `lifecycle-kit/checks/check-gap-inbox-neutrality.gate` — couples (delta 3).
- `scripts/git-hooks/pre-commit` — regenerated (deltas 2 and 3).
- `.workflow/release-declarations.md` — Behavior changes (delta 2).
- `docs/check-graph.html` — generated artifact, regenerated (deltas 2 and 3).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).

## Retired spellings

- None — no delta retires a name. Delta 3 deletes couple tokens whose path stays live as a derived
  trigger, so every deleted string still names the path `registry::knob_files` derives for that
  member, whether or not a consumer has adopted an override file there — `scripts/doctrine-config.knobs`
  is such a path with no file at it today.

## Definition of Done

- [ ] **Causal completeness** — `registry::knob_files` has a reachable producer and four named
      readers, and its unit test holds both halves.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — the derivation lands in §The `# graph:` manifest and the
      arm in §lib/gate.sh. The measured reader set is evidence, and it stays in this file's history
      and the survey record rather than on the kit surface.
- [ ] **Amendment deleted**, and the entry moves to Done, dropping its `[spec:]` tag.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any post-delta selection missing a measured reader that build cannot derive,
      and whether `check-gate-fixture-coverage` takes top-level flags, if it lands as a gap.
