# SPEC amendment: manifest-span

**An amendment body's manifest is told from a prose mention of the grammar by the shape of the span, never by where it sits in the sentence.** Today the reader beside `validate_amend_manifest` in `native/src/gates/graph.rs` takes every inline code span whose text opens with the manifest header (the `# graph:` token and one space) as a manifest and holds it to the four required keys. So a sentence naming one key mid-paragraph reds assertion G on a missing `dir=`, `valve=` and `tier=` and an empty `couples=`, and the author rephrases around the token. The SPEC states no grammar for which spans count, so the reader's behaviour is the only statement of it.

**Measured at authoring.**

- An untracked root probe amendment carrying one mid-sentence span, the header followed by a bare `couples=`, turns `bash gate-sdk/bin/run-gates.sh --only check-graph` red on four AMEND-MANIFEST lines: missing `dir`, `valve` and `tier`, and an empty `couples`.
- `git log --all -p -S'# graph: ' -- 'SPEC-*.md' '*/SPEC-*.md'` yields 15 distinct manifest spans over every amendment this repository has carried. All 15 open with a valued first key (a `couples=` form in 14, the bad fixture's `mode=partial bogus=1` in the 15th), so the grammar delta 1 states admits each. They sit after a verb, after a colon label, inside parentheses, two to a line, and at a line's start, which is why a positional grammar (a span owning its whole line or list item) was ruled out: it drops several of them and silences the bad fixture's inline cases.
- The reader walks every `SPEC-*.md` under the root through `amendment_findings`, a kit directory's amendment included.

## What changes

### (1) §check-graph states which amendment-body spans are manifests {design-bearing} {user-facing: amendment-manifest-prose-span's deliverable, the SPEC states which spans are manifests so a mid-sentence mention is told apart from an embedded one}

gate-sdk/SPEC.md §check-graph. **Not yet applied.** The second paragraph's last sentence becomes:

> An amendment-body manifest is held to the glob grammar but not to the vocabulary or trigger parity, since the gate it describes is unbuilt; parity re-fires through the registry once the gate lands.

and a paragraph follows that paragraph:

> **Which amendment-body spans are manifests.** Assertion G reads every `SPEC-*.md` under the scan root. Outside a fence, a manifest is an inline code span opening with the manifest header whose first word after it is a `<key>=<value>` pair with a non-empty value, wherever the span sits in its sentence; two on one line are read apart. A span whose first word is a bare key or prose is a mention and is never read, so a sentence may name `couples=` without meeting the four required keys. In a fence, a manifest is a line beginning with the header, and a `proto` fence is never read. **Honest limit:** an inline manifest whose first key is bare is read as a mention and goes unchecked, so a manifest meant to be held leads with a valued key or sits in a fence.

### (2) The amendment reader takes an inline span only when its first word is valued {mechanical} {user-facing: amendment-manifest-prose-span's deliverable, the reader follows the stated grammar}

`native/src/gates/graph.rs`, `extract_amend_manifests`. After finding an inline span that opens with the header, the reader keeps it only when the span's first whitespace-separated word after the header splits at its first `=` into a non-empty key and a non-empty value. A dropped span is skipped and the scan resumes after its closing backtick, so a second span on the line is still read. The fence arm, the `proto` skip and `validate_amend_manifest` are untouched.

The extractor's unit test gains rows: a bare first key (dropped), a bare key then prose (dropped), a valued first key mid-sentence after a verb (kept), a valued first key outside the vocabulary such as `mode=partial bogus=1` (kept, so its findings stand), a dropped span then a kept one on one line (the second kept), and a fence line with an empty first value (kept).

**Inferred, cannot run before build:** each row's verdict — the reader that decides them lands in this delta.

### (3) The fixture pair covers a mention and a mid-sentence manifest {mechanical}

`gate-sdk/gate-tests/check-graph/`, run through `--amend-only`.

- `gate-sdk/gate-tests/check-graph/good/SPEC-example-gate.md` gains a sentence naming `couples=` and `dir=` mid-sentence in spans opening with the header and a bare key. It reds today and must clear.
- `gate-sdk/gate-tests/check-graph/bad/SPEC-example-gate.md` gains a list item placing a manifest mid-sentence after a verb, valued first key, with an illegal `tier=` value; `bad/expect.txt` gains the one line for that value. The expectation is a conjunction of lines (§Fixture-pair discipline), so no existing line moves.

**Inferred, cannot run before build:** that the new good lines clear and the new bad line matches — delta 2's reader decides both.

### (4) The site mirror follows {mechanical}

`docs/gate-sdk/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 1.

## Producers and consumers

- **The kept and dropped spans.** Producer: an amendment author writing a `SPEC-*.md` anywhere under the scan root, read by `amendment_findings` over the shared pruned walk. Enabling configuration: `check-graph` is a `gates.list` member here and in every consumer taking gate-sdk's registry; assertion G runs in the whole-tree arm and in `--amend-only` alike. Consumer: `validate_amend_manifest`, by direct call. No field, state or knob is added.
- **Narrowing, with its red condition (point 5).** The corpus loses the spans with a bare or prose first word. The extractor's only reader is assertion G, by `grep -rn "extract_amend_manifests\|validate_amend_manifest\|amendment_findings" native/src`; its red condition is a kept span failing a key, value, glob or repeat check, monotone in the kept set. It has no reds-on-empty, count or floor arm, so a dropped span can only remove findings. The one cost is the honest limit delta 1 states.
- **Obligation on members (point 6).** None: the change obliges no corpus member.

## Existing sections updated

Roster by `grep -n "amendment body\|amendment manifest\|embedded in a" gate-sdk/SPEC.md` and `grep -rn "graph: " canon-kit/templates lifecycle-kit/templates canon-kit/SPEC.md lifecycle-kit/SPEC.md` over the tracked tree; no template tells an author how to embed a manifest.

- `gate-sdk/SPEC.md` — §check-graph's second paragraph and the paragraph after it (delta 1).
- `native/src/gates/graph.rs` — `extract_amend_manifests` and its unit test (delta 2).
- `gate-sdk/gate-tests/check-graph/good/SPEC-example-gate.md`, `gate-sdk/gate-tests/check-graph/bad/SPEC-example-gate.md` and `bad/expect.txt` (delta 3).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 4).

## Retired spellings

- None — the change narrows which spans the reader takes; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition is touched.
- [ ] **Merged with no information lost** — §check-graph reads as one section stating the span grammar beside the glob grammar.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration while sibling gate-sdk amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery and gate-sdk's fixture suite in the merging batch.
- [ ] **The entry is done** — `amendment-manifest-prose-span` moves to Done in the merging commit, before the drain stage.
