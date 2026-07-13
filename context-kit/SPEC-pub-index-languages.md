# SPEC amendment: pub-index-languages

Supersedes the standing single-extractor ruling in context-kit/SPEC.md
§Index-first reading ("a speculative plugin interface for extractors nobody
has written yet would be scaffolding"). The demand gate that paragraph
implied has now fired: the anticipated second adopter's tree carries a
substantial TypeScript surface beside its Rust one, and its vendored
`pub-index.sh` copy cannot index it (identity on record in the operator's
local brief — the established masking pattern). One known consumer, one
known language: this ships the mechanism that demand names, not a framework
for languages nobody asked for.

## What changes

**1. `pub-index.sh` becomes a dispatcher over per-language extractors.**
Output format is unchanged (per-file header with count, `kind name :line`
rows, kind-then-name sort); the language-specific parts — the file glob set
and the line-grammar extraction — move into extractor files. An extractor is
a sourced bash file defining exactly two names: a `PUB_LANG_GLOBS` array
(the find globs, e.g. `*.rs`) and a `pub_lang_extract <file>` function
emitting `kind name lineno` rows; the dispatcher owns traversal, the prune
set (`.git`, `target`, `node_modules`, `dist`, `build`), sorting, and
formatting. The exact contract prose lands in the SPEC's §Index-first
reading at merge.

**2. Registry-style resolution, the gate-sdk convention.** Shipped
extractors live at `context-kit/lib/pub-lang/<lang>.sh`; a consumer
extractor of the same basename shadows the kit's, resolved from
**`CONTEXT_KIT_PUB_LANG_DIR`** (default `${GATE_SDK_GATES_DIR:-scripts}/pub-lang`)
first — the gates.list consumer-first precedent. **`CONTEXT_KIT_PUB_LANGS`**
(array) names the enabled set; its default is every shipped extractor,
derived from the `lib/pub-lang/` roster at run time, never maintained as a
list (derivation-first). Both knobs join the SPEC's knob roster.

**3. Two shipped extractors.**

- `rust.sh` — the existing grammar moved verbatim; the existing golden
  (`index-tests/expected/pub-index.txt`) must stay byte-identical, which is
  the no-regression assertion for the dispatcher refactor.
- `ts.sh` — TypeScript: `export`-declared `function` (incl. `async`),
  `class`, `interface`, `type`, `enum`, `const`/`let`/`var`, and
  `export default`; globs `*.ts` and `*.tsx`; `.d.ts` files are included
  (a declaration file is public surface by construction). Grep-grade line
  grammar like the Rust extractor — re-exports (`export { x } from`) and
  multi-line declarations are stated honest limits, not parsed.

**4. Surfaces reworded** (absorbs the pub-index-rust-default-framing debt
task, whose slug closes with this promotion): the context-kit README tool
line and the session-context nudge line (kit template *and* this repo's
`scripts/session-context.sh` copy) present the tool as "public API surface
(per-language extractors; ships rust, ts)" — Rust demoted from identity to
shipped default. context-kit/SPEC.md §Index-first reading's pub-index bullet
is rewritten around the extractor contract and knobs, replacing the
superseded write-your-own paragraph.

**5. Tests.** `index-tests/corpus/sample.ts` joins the corpus with a golden
exercising every kind the TS grammar claims; the runner gains the case
beside the Rust one. A consumer-shadowing case (a scratch
`CONTEXT_KIT_PUB_LANG_DIR` overriding `rust.sh`) exercises resolution order.
`bin/run-index-tests.sh` itself registers as a hand-listed evidence-kit
validate suite in `scripts/evidence-config.sh` (`index_tests`, the `demo`
precedent) — the align audit found it registered in no validate suite, so
the golden this refactor leans on as its no-regression assertion had no
automated consumer (runner verified green at align, pre-refactor).

**Ruled out:** AST/tree-sitter parsing — grep-grade extraction is the tool's
altitude and its portability guarantee (bare bash + coreutils, Tier one);
per-language plugin *packaging* or a manifest format — the registry is a
directory of sourced bash files, nothing more; auto-detecting languages from
tree contents — the enabled set is explicit config with a derived default,
so an unexpected vendored `.ts` under a Rust tree cannot silently change
tool output.

## Producers and consumers

- **`CONTEXT_KIT_PUB_LANGS`** — producer: consumer config
  (`context-config.sh`, the existing config-file mechanism footprint.sh
  already sources; unset in this repo — the derived default covers it);
  reader: the dispatcher's enabled-set resolution at invocation.
- **`CONTEXT_KIT_PUB_LANG_DIR`** — producer: consumer config; reader: the
  dispatcher's extractor resolution, before `lib/pub-lang/`.
- **`PUB_LANG_GLOBS` / `pub_lang_extract`** — producer: each extractor file;
  consumer: the dispatcher (traversal and per-file extraction respectively).
  Two names, both read every run; no other extractor-file surface is
  contract.
- **The TS extractor's output** — consumer: any session reading a TypeScript
  consumer's API surface via the nudge line; exercised by the new golden.

## Existing sections updated

- context-kit/SPEC.md §Index-first reading — the pub-index bullet rewritten
  (contract, resolution, shipped set); the superseded single-extractor
  paragraph replaced by the demand record.
- context-kit/SPEC.md knob roster — the two new knobs.
- context-kit/SPEC.md §Testing — the TS and shadowing cases.
- context-kit/README.md — the tool line (change 4).
- context-kit/templates/session-context.sh + this repo's
  `scripts/session-context.sh` — the nudge line (change 4).
- scripts/evidence-config.sh — the runner's validate-suite entry
  (`index_tests`; change 5).
- Docs mirror regenerated for the touched SPEC/README
  (`scripts/gen-docs-mirror.sh --write`).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component beyond its siblings' own (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (the write-your-own `<lang>-index` guidance); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
