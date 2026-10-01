# SPEC amendment: consumer-value-literal

**A new gate, `check-consumer-value-literal`, holds the port-candidate rule that every consumer value a kit-shipped gate reads is a knob, never a crate literal.** It covers the crate modules of every gate whose descriptor ships in a kit. Today the rule is a sentence in §The port-candidate criteria (the paragraph *Every consumer value a member reads is a knob with one producer*) and a clause of §Consumer payload, and no gate holds it. Instances surface by hand: `installer-graph-artifact-literal` and the release-note section names.

**The corpus is ruled, by operator direction 2026-10-02 (lead-relayed, not a ruling): the kit-shipped gates' modules.** That was option C of three: build the kit-shipped half this iteration, and file the other half as its own deferred entry, `withheld-gate-literal-knobs`. That entry covers every gate module, including the gates withheld to the publisher's gates dir, and it keeps the 2026-09-27 direction "avoid any hard code in gates … benefits from them itself" on record for the repo-only half. The two grounds:

- §Consumer payload asks configurability of a gate that ships.
- §The port-candidate criteria's own ground is that a literal "would publish one project's configuration as everyone's mechanism". A withheld gate publishes nothing, because no adopter receives its descriptor.

**Measured at authoring** by a read-only sweep of `native/src/gates/*.rs`, each file cut at its test module. The extraction was a throwaway string-literal scanner, and the class labels are hand judgement.

- **Population.** 7674 literals: 72 consumer values, 71 borderline, and the rest mechanism (output prose, knob names, tool syntax, kit constants).
- **Where they sit.** 62 of the 72 consumer values sit in the 26 gates whose descriptors are in `scripts/`. Among kit-owned modules the consumer values are `install_disposition` (a crate path) and `lesson_disposition` (a queue heading). The borderline rows there are mostly literals a `// spec:` ground already rules kit constants: `gate_exemption_tasks`' section names, `root_tiering`'s built-in fallback, and the amendment and doctrine headings.
- **Which predicate works.** Over the whole corpus, *a whole literal naming a tracked path outside every kit root, or equal to a multi-word heading of a tracked markdown file outside them* hits 66 literals: 55 consumer values, 8 borderline and 3 false. A path token anywhere inside a literal hits 143, with 88 false, almost all remedy prose. Heading equality without the multi-word cut hits 314, with 304 false: kit and gate names that the docs mirrors carry as headings.
- **Member to module.** Every one of the 150 descriptors resolves to `native/src/gates/<module>.rs`. The probe was a scratch script run through `--scratch-run` over `*/checks/*.gate scripts/*.gate`. The build-generated `GATE_MODULES` table (`native/src/registry.rs`) already maps each member to its module.
- **The withheld pattern.** `check-release-assets` and `check-release-change-declared` are withheld gates whose rule sections live in this SPEC. Their descriptors are in `scripts/`, and their rules stay in the crate.

## What changes

### (1) The gate module: `native/src/gates/consumer_value_literal.rs`

A compiled gate with its registry row in `native/src/gates/mod.rs` {design-bearing}.

**The corpus is derived, never listed.** It is built in three steps:

- **Members.** Take every registry member in `GATE_MODULES` whose descriptor `registry::resolve` finds under a kit root (`walk::kit_roots`), over `registry::resolve_dirs`. A member whose descriptor resolves to the consumer's gates dir is withheld and left out, and so is a member that resolves nowhere.
- **Files.** The corpus is the union of those members' `registry::module_files` cuts. That is each member's own module, its direct first-party imports, and the imports of an `emit/` module among them, less the universal layers.
- **Exclusions and cut.** Any file under `<GATE_SDK_NATIVE_SRC>/knobs/` is dropped, because the knob tables are the one sanctioned producer of a consumer value (§The knob file). Each file is read up to its test module, the first `#[cfg(test)]` line followed by `mod tests {`, which is the cut `check-door-binding`'s Rust reading makes.

**The literal reader** walks Rust source. It takes:

- `"…"` with backslash escapes;
- raw strings `r"…"` and `r#…"…"#…`, and the `b`-prefixed forms of each;
- each literal's content as written, attributed to its opening line.

It skips `//` and `/* … */` comments, char literals and lifetimes. No reader of Rust string literals exists in the crate today, so this one is new.

**A literal is a consumer value** when either arm holds:

- **The path arm.** Its whole content is a repo-relative path that `git ls-files` lists, or a directory holding a listed file, and its first component is no kit root. A kit's own files are kit vocabulary.
- **The heading arm.** Its content matches the text of a heading, a `#`-led line outside a fence, in a tracked `.md` file outside every kit root and outside the shared prune set (§lib/gate.sh). It matches in one of two shapes:
  - a literal carrying the heading's marker (`## Done`) matches whatever the heading's word count;
  - a bare literal matches only a heading of two or more words.

  The word cut is what keeps gate and kit names, which the docs mirrors carry as one-word headings, out of the arm.

**The valve** is `// consumer-value-exempt: <reason>` on the literal's opening line or the line above, with the reason mandatory. An empty reason is malformed and reds as its own finding, and the reason is echoed into the red line's detail, on the `door-contributor:` convention (guard-kit/SPEC.md §check-door-binding). A literal a `// spec:` ground already rules a kit constant takes the valve citing that ground. The ground stays where it is, and the valve is the gate's discharge.

**The verdict lines.**

- **Red** names the file, the line, the literal, and the arm. The path arm names the tracked path. The heading arm names the heading and the file it was found in. The `help:` line gives the two repairs: read the value from a knob in the owning kit's table with the literal as its default (§The knob file), or declare the site with the valve.
- **The clean line** prints the members swept, the files read, the literals read and the sites valved. It is a report, with no floor on any count.
- **Exit 2** when `GATE_SDK_NATIVE_SRC` is empty or names no directory, when `git ls-files` fails, when a corpus file cannot be read, or when no kit-shipped member resolves. A gate that reads no crate cannot say the crate is clean.

### (2) The descriptor, registration and fixture pair

`scripts/check-consumer-value-literal.gate`, `scripts/gates.list` and `scripts/gate-tests/check-consumer-value-literal/{good,bad}/` {mechanical}. The gate's subject is the publisher's crate source, which no adopter holds, so §Consumer payload withholds it: the descriptor sits in the consumer's gates dir, as `check-release-assets`' does, and the rule stays in the crate. The descriptor is:

```
# graph: couples=native/src/*.rs,*.gate,*.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-consumer-value-literal — every literal a kit-shipped gate's crate modules carry that names a tracked consumer path or heading is a knob or a declared site
```

The `couples=` field is a calibration build may tighten against `check-graph` and `check-reads-couples`. A path joining or leaving the tracked set can flip a verdict, and no trigger reaches that, so the full battery is the catch.

The fixture pair builds a crate source and a kit root of its own. The case's knob file points `GATE_SDK_NATIVE_SRC` at it, and the module files carry real member names, since `GATE_MODULES` is compiled.

- **`bad/`** carries an unvalved path literal, an unvalved heading literal in each shape, and an empty-reason valve.
- **`good/`** carries:
  - the same literals valved;
  - a kit-internal path;
  - a literal inside the test module;
  - a literal in a module whose member's descriptor sits in the case's gates dir.

### (3) The valve joins this repo's comment-tier extras

`scripts/canon-config.knobs` gains `CANON_KIT_COMMENT_MACHINE[] = consumer-value-exempt:` with a `# spec:` comment naming this section {mechanical}. canon-kit/SPEC.md §check-comment-tier admits a site valve to the built-in roster only because it is kit mechanism. This gate is the publisher's own, so its valve is consumer vocabulary, which is what the `CANON_KIT_COMMENT_MACHINE` extras exist for. The `.rs` comment surface is already governed here (`CANON_KIT_COMMENT_SURFACE[] = **/*.rs`).

### (4) The census: every finding gets a knob or a valve

{design-bearing} Build runs the gate over the tree and disposes of every finding in the commit landing delta 1. Each finding either reads the value from a knob or takes the valve. **No finding is re-expected.**

**This act depends on the sibling debt entry `gate-customer-value-audit`, gate-sdk's slice, in two cases:**

- **The audit lands in an earlier batch, or earlier in the same batch.** The census reads the corpus after its verdicts. A gate-sdk gate it withholds has left the corpus by derivation. A gate it makes generic has lost its literal before the census runs.
- **This unit lands first.** The census disposes of the current corpus. The audit's later verdicts then remove findings this unit valved, and those valves stay as harmless dead declarations. The audit's commit deletes any valve its own change made moot.

**Every member's satisfying value, from the sweep's kit-owned rows.** These predict the gate's run, and the gate's first run is the binding roster. The arms read the tree as it stands at build, so a row may move.

- `install_disposition.rs` — the crate path `native/src/installer/recipe.rs`: join `GATE_SDK_NATIVE_SRC` with `installer/recipe.rs`.
- `lesson_disposition.rs` — the heading `## Lessons Learned`: the valve, citing queue-kit's section format. No knob names that section alone, since `QUEUE_KIT_REQUIRED_SECTIONS` rosters it among six (`native/src/knobs/queue_kit.rs`), and minting one would be a new name this amendment does not carry.
- `gate_exemption_tasks.rs` — the queue section names: the valve, citing its own `// spec:` ground that these are literals, never knobs, on fail-open grounds.
- `root_tiering.rs` — the built-in root allowlist: the valve, citing its `// spec:` generic-fallback ground.
- `amendment_*` and `doctrine_registration.rs` — the kit-constant headings, where a tracked file outside the kit roots carries the same heading: the valve, citing their kit-constant grounds.
- `merge_attrs.rs` (`.gitattributes`), and each kit-own `README.md` joined under a kit root (`door_binding`, `docs_link_convention`, `docs_restatement_parity`, `docs_mirror_fresh`, `kit_roots_dialect`): the valve. Each names a git file or a kit file name, not a consumer's value.

**Inferred, cannot run before build:** that the gate's first run reds exactly these rows and no others — the gate is delta 1's and does not exist until build lands it. Every finding it adds takes one of the same two forms, and the valve is always available, so every member has a satisfying value.

### (5) The owning sections state the gate

gate-sdk/SPEC.md {mechanical}. **Not yet applied.**

In §The port-candidate criteria, the paragraph *Every consumer value a member reads is a knob with one producer, never a crate literal* gains a closing sentence:

> For a gate that ships, §check-consumer-value-literal holds the crate-literal form: a literal naming a consumer's tracked path or heading.

In §Consumer payload, the sentence *Whose surface a subject is stays judgement, which no gate holds* becomes:

> Whose surface a subject is stays judgement, which no gate holds; the configurability half's crate-literal form is held by §check-consumer-value-literal over the shipped set.

A new section, `### check-consumer-value-literal`, follows §check-release-assets. It states the following, re-phrased from delta 1, with no grounds the two paragraphs above already carry:

- the invariant;
- the derived corpus and the knob-table exclusion;
- the two arms and the multi-word cut;
- the valve;
- the red, clean and exit-2 lines;
- the withheld disposition.

Its **honest limits**:

- A consumer value that is neither a tracked path nor a heading is out of reach. That covers vocabulary such as a release channel's label, product strings, a single-word bare heading, and a path the tree does not hold yet.
- So is a templated literal such as `"{}/release-declarations.md"`.
- So is any literal in a universal layer, `gates/mod.rs`' walk-root declarations among them.

### (6) Every projection the new gate moves follows

{mechanical} Build regenerates every generated projection the battery reds on after delta 2, each with the command its freshness gate prints. Among them:

- the site mirror `docs/gate-sdk/SPEC.md`, with `--emit docs-mirror --write`, in the commit landing delta 5;
- the enforcement map;
- the graph artifact.

## Producers and consumers

- **The verdict.** Producer: the new member, at every `--run` and at commit through the generated hook on its `couples=` (`scripts/gates.list` registers it). Enabling configuration: a tree carrying the crate source at `GATE_SDK_NATIVE_SRC`, which this repo does and no vendored tree does, because the payload ships the binary. Consumer: the committing session through the output contract.
- **The valve token.** Producer: a source comment a session writes at a site. Consumers:
  - the new gate, which reads the token, its reason and its window;
  - `check-comment-tier`, which admits the comment through this repo's `CANON_KIT_COMMENT_MACHINE` extras (delta 3).

  The token is minted into the roster-holding reader in the same commit.
- **Readers of the derivation it reuses:** `GATE_MODULES`, `registry::resolve`, `registry::resolve_dirs`, `registry::module_files` and `walk::kit_roots`, each called as it is, with no change to their contracts.
- **Registry-data readers of the new row:** `check-reads-couples` (its declared walk roots), `check-gate-substrate-parity`, `check-crate-arms`, and the enforcement map. Each holds the row by its existing rule, and build declares what they ask.
- **Red conditions.** The gate reds on a finding and exits 2 on a corpus that resolves no member. That second condition is a reader that refuses on finding none. The audit's withholding verdicts narrow the corpus, but only gate-sdk's own gates are in that slice and nine other kits ship gates (the audit tally on `gate-customer-value-audit`), so the corpus cannot empty. The clean line's counts are a report, with no floor.
- **No new knob, state or event.**

## Existing sections updated

Roster produced by `grep -n "consumer value\|crate literal" gate-sdk/SPEC.md`, by the canon-kit read of §check-comment-tier's site-valve paragraph, and by the census sweep above.

- `native/src/gates/consumer_value_literal.rs` and its row in `native/src/gates/mod.rs` (delta 1).
- `scripts/check-consumer-value-literal.gate`, `scripts/gates.list` and `scripts/gate-tests/check-consumer-value-literal/` (delta 2).
- `scripts/canon-config.knobs` (delta 3).
- The kit-owned gate modules the census reds, starting with the rows under delta 4 (delta 4).
- `gate-sdk/SPEC.md`: §The port-candidate criteria, §Consumer payload, and the new §check-consumer-value-literal (delta 5).
- `docs/gate-sdk/SPEC.md` and every other projection the battery reds on (delta 6).

## Retired spellings

- None — the change adds a gate and a valve token; the census moves literals into knob reads without renaming any knob or file.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
