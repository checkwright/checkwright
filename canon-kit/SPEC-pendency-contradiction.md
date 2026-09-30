# SPEC amendment: pendency-contradiction

**A file can call one thing landed in one section and pending in another, and nothing reads both.** gate-sdk/SPEC.md once said a second port "lands after `native-artifact-publish-path` and `native-artifact-install-path`". About seventy lines later it said criterion 5 is "implemented by `native-artifact-publish-path` and `native-artifact-install-path`". Both had landed the day before. The close audit that swept for pendency after landing missed it, because that sweep compares prose with the tree and its reach is whichever files the sweeper opens. This shape needs no tree: the file contradicts itself, so it falsifies itself.

**The construction vocabulary is the whole gate, and its config shape follows canon-kit's bundled-vocabulary rule.** "Waits on", "lands after", "implemented by" and "landed" are generic English, and §Layout and configuration rules that generic English is kit-shippable. So each vocabulary ships a bundled set and takes an `_EXTRA`, as `CANON_KIT_TEMPORAL_MARKERS` does, and a consumer wanting a member gone replaces the base array. The optional-document shape of `check-graph`'s vocabulary is refused here, because that shape fits a vocabulary no two consumers share, such as a layer roster. With both sets empty the gate skips clean, which is the off position among the calibrations.

**Adjacency answers the entry's open question.** The question was whether "X landed, Y still waits on it" trips a per-slug predicate. A construction binds only the code-span tokens directly in its slot: after "waits on", or before "landed". "Waits on it" binds nothing, so that sentence pair is clean. "`Y` still waits on `X`" beside "`X` landed" binds `X` both ways, and that is a real contradiction, since nothing waits on what has landed. With the binding that tight, the predicate can be per file, which is the only grain that catches a pair seventy lines and two sections apart.

**The subject is any code-span token, not a queue slug.** check-provenance-seam bars queue slugs from kit SPECs, and the incident's slugs were stripped from gate-sdk/SPEC.md afterwards. A consumer's own docs cite work items, gates and arms as code spans, and the contradiction is the same whatever the token names. The gate therefore reads no queue.

**Measured at authoring.** The sweep ran over the tracked kit SPECs, READMEs, CLAUDE.md and non-mirror `docs/*.md`, with paragraphs joined across wraps and fences skipped. It found one sentence carrying a pending construction and a code span, context-kit/README.md's "wait on a step of yours: `check-settings-pins` skips clean until …", where no span sits in the slot. It found no (file, token) pair bound both ways. The incident was confirmed at `git show ec628234^:gate-sdk/SPEC.md`.

## What changes

### (1) The gate

A new canon-kit gate, `check-pendency-contradiction`, born native {design-bearing}. Its parts:

- `native/src/gates/pendency_contradiction.rs`;
- `canon-kit/checks/check-pendency-contradiction.gate`, carrying `check-manifest-temporal.gate`'s manifest line unchanged, since both read the manifest set, then `# install: zero-config` and its `# spec:` pointer. The vocabulary knobs take no `knob:` token, on the ground §Layout and configuration gives the fence-program pair: they are vocabularies, not walk filters;
- a `good/`+`bad/` fixture pair.

**Corpus.** The manifest set (`spec::manifest_files`, §The shared spec adapters), read per file over a blank-line paragraph join so a construction broken by a wrap still matches. Fenced blocks and HTML comments are skipped.

**Constructions.** Each member of `CANON_KIT_PENDENCY_PENDING` and `CANON_KIT_PENDENCY_LANDED` (each base unioned with its `_EXTRA` through `spec::vocabulary`) is a phrase carrying one `{}` slot. The phrase is matched case-insensitively with whitespace folded, in prose outside code spans, and a word boundary sits at each end. So a phrase quoted as a span, as a knob bullet quotes its defaults, is never a construction.

The slot binds a **span list**: one inline code span, or several joined by `, `, ` and `, `, and ` or ` or `. A span holds an identifier-shaped token, one run of `[A-Za-z0-9._/-]`. A slot after the phrase binds the list that follows, and a slot before binds the list that precedes. Nothing else binds, so a pronoun, a noun phrase or a span one word away binds nothing.

**Assertion.** Per file, a token bound by a pending construction and also by a landed construction is red. There is one finding per token, naming the file, the token and the line of each first binding. The help line reads *the file says both: make the pending sentence past, or drop it*.

**Valve.** `pendency-exempt: <reason>` on a binding's line or the one above, in the shared exempt window, drops that binding. The reason is mandatory.

**Clean and exit 2.** The clean line counts files, pending bindings and landed bindings. Exit 2 comes from:

- a vocabulary member that does not carry exactly one `{}`, or carries nothing beside it (the kit's table validator, at every canon-kit gate, naming the knob);
- an unreadable manifest file.

Either class empty, base and extra together, is a clean skip whose line names the empty knob.

**Bundled sets.**

- Pending: `waits on {}`, `wait on {}`, `waiting on {}`, `lands after {}`, `land after {}`, `blocked by {}`, `until {} lands`, `once {} lands`, `{} does not exist yet`, `{} is not yet`, `{} has not landed`.
- Landed: `implemented by {}`, `built by {}`, `delivered by {}`, `{} landed`, `{} has landed`, `{} ships`, `{} shipped`.

**Fixtures.**

- `bad/` carries the incident's shape: a pending coordinated list in one section and a landed one in another, three paragraphs apart, one of them broken by a wrap.
- `good/` carries "`X` landed; `Y` still waits on it", a span one word from the phrase, a fenced decoy, a valved binding, and a token pending in one file and landed in another.

### (2) §check-pendency-contradiction

canon-kit/SPEC.md gains a section after §check-manifest-temporal {mechanical}. **Not yet applied.**

> ### check-pendency-contradiction
>
> Invariant: no governed manifest file binds one code-span token both to a pending construction and to a landed one. The contradiction is internal, so the check needs no tree comparison and no judgment of what is live: a file calling one span both awaited and delivered falsifies itself. The corpus is the manifest set (§The shared spec adapters) over a paragraph join, fences and HTML comments skipped.
>
> A construction is a `CANON_KIT_PENDENCY_PENDING` or `CANON_KIT_PENDENCY_LANDED` phrase with one `{}` slot, matched case-insensitively in prose outside code spans. The slot binds the code span, or the list of code spans joined by commas, `and` or `or`, directly after or before it and nothing else. Adjacency is what keeps a sentence pair clean where one span landed and a second still waits on "it", while a second sentence naming the landed span in the waiting slot reds. It is also what lets the grain be the whole file, the only grain that reaches a pair sections apart. Any code span is a subject rather than only a queue slug, since a consumer's prose cites work, gates and arms alike and the contradiction does not depend on which. The fixture pair states both sides of the boundary, and this section binds no span in either class, since it is governed by the gate it describes.
>
> Red is one finding per token bound both ways, naming the first line of each binding. The valve is `pendency-exempt: <reason>` in the shared exempt window. A member without exactly one slot is malformed config (exit 2), and a class with no members is a clean skip. `tier=precommit`, `install: zero-config`.
>
> **Honest limits.** A contradiction spelled without a code span, or through a construction outside the configured sets, passes. So does a pending claim contradicted by the tree rather than by the file, which stays the close-stage audit's. A pending sentence and its landed correction in different files pass by design.

### (3) The knobs

canon-kit/SPEC.md §Layout and configuration and `native/src/knobs/canon_kit.rs` {mechanical}. **Not yet applied.**

After the `CANON_KIT_TEMPORAL_MARKERS` bullet:

> - `CANON_KIT_PENDENCY_PENDING` / `CANON_KIT_PENDENCY_LANDED` — arrays of construction phrases, each carrying one `{}` slot, that `check-pendency-contradiction` reads as awaiting and as delivered the code spans in the slot. Defaults are bundled generic-English sets (`waits on {}`, `lands after {}`, `{} does not exist yet` …; `implemented by {}`, `{} landed`, `{} ships` …; `--emit knob-roster` prints each whole). A member without exactly one slot is malformed config.

In the bundled-vocabulary paragraph ("**A bundled vocabulary is extended, never restated.**"), the two bases join the list, and their extras, `CANON_KIT_PENDENCY_PENDING_EXTRA` and `CANON_KIT_PENDENCY_LANDED_EXTRA`, join the list of matching extras. That paragraph's closing rule ("A temporal or authority marker set whose base and extra are both empty is malformed config") is unchanged, because an empty pendency class is the off position.

The table gains the four indexed rows and a slot validator. The member's knob list in `native/src/gates/mod.rs` gains the four names.

### (4) The new-gate fan-out

The registration and projections docs/site-architecture.md §Generated projections and their freshness gates lists for a new gate {mechanical}:

- `scripts/gates.list` registers it, beside `check-manifest-temporal`;
- `canon-kit/README.md`'s gate roster gains the line `check-pendency-contradiction  # no file calls one code span both pending and landed`;
- `canon-kit/smoke/install.sh`'s registry heredoc registers it;
- `.workflow/validate-baseline.txt` gains `gates check-pendency-contradiction pass`;
- the generated projections are regenerated by the commands that page lists: the on-site mirror, `docs/enforcement.md`, `docs/value.md`'s rollup, `docs/check-graph.html` and the hooks, and the `.workflow/surface-ceiling.txt` rows.

### (5) The release declaration

`.workflow/release-declarations.md` {mechanical}, under Tightened gates:

> - `check-pendency-contradiction` — new, zero-config: a governed manifest file that calls one code span both pending (`waits on`, `lands after`, `does not exist yet` …) and landed (`implemented by`, `landed`, `ships` …) reds. Make the pending sentence past or drop it; a deliberate pair takes `<!-- pendency-exempt: <reason> -->`, and extending or replacing the phrase sets is `CANON_KIT_PENDENCY_*` in your canon-config.knobs.

**Inferred, cannot run before build:** the gate reds nothing on the live tree, `docs/posts/*.md` included, which the authoring sweep did not read — the gate does not exist until delta 1 lands, and it is the oracle; a live finding is fixed or valved in the landing commit.

## Producers and consumers

- **The verdict.**
  - Producer: the new member, on the generated pre-commit hook, `run-gates.sh` and CI.
  - Enabling config: the bundled defaults, so every consumer registering it runs it. `init` registers a zero-config gate.
  - Consumer: the committing session through the output contract, which reads each finding's two lines at the scan. Nothing persists.
- **The vocabulary knobs.**
  - Producer: the knob table and the consumer's knob file.
  - Consumer: this member alone, through `spec::vocabulary`.
  - Every field (a phrase, its slot position) is read at the one match.
- **Roster-holding readers.**
  - `check-gate-substrate-parity` holds the registry and smoke heredoc against the binary.
  - `check-gate-fixture-coverage` holds the pair.
  - `check-kit-enum` and `check-graph` read the descriptor.
  - `check-kit-ref-liveness` and `check-docs-cmd` resolve the four names through the static table.
  - `check-knob-citation` and `check-knob-default-coupling` read the new bullet, which names its sets as `CANON_KIT_PROSE_TELL_PHRASES`' bullet does.
  - Each is an update target through delta 4 or reads the table directly.

## Existing sections updated

- `native/src/gates/pendency_contradiction.rs`, `canon-kit/checks/check-pendency-contradiction.gate`, `canon-kit/gate-tests/check-pendency-contradiction/good/` and `bad/` (delta 1).
- `canon-kit/SPEC.md`, the new §check-pendency-contradiction (delta 2) and §Layout and configuration (delta 3).
- `native/src/knobs/canon_kit.rs` and `native/src/gates/mod.rs` (deltas 1 and 3).
- `scripts/gates.list`, `canon-kit/README.md`, `canon-kit/smoke/install.sh` and `.workflow/validate-baseline.txt` (delta 4).
- `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md`, `docs/enforcement.md`, `docs/value.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit` and `.workflow/surface-ceiling.txt`: the generated projections (all deltas).
- `.workflow/prose-bound-ceiling.txt`: the `canon-kit/SPEC.md` row, re-stamped to the count `check-prose-bounds` prints if deltas 2 and 3 move it (deltas 2 and 3).
- `.workflow/release-declarations.md` (delta 5).

The roster came from the file list of the commit that landed `check-prose-bounds` (`git show --stat 66435b97`), read against docs/site-architecture.md's new-gate fan-out, and from `grep -n 'TEMPORAL_MARKERS' canon-kit/SPEC.md native/src/knobs/canon_kit.rs`.

## Retired spellings

- None — the amendment adds a gate and four knobs, and renames nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate and its knobs.
- [ ] **Instruction surfaces: instruction only**: the help line and the README roster line state the rule and nothing else.
- [ ] **Merged with no information lost**: the new section carries the adjacency ground and the refused vocabulary shape.
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `intra-file-pendency-contradiction-scan` moves to Done in the landing commit, before the drain stage.
