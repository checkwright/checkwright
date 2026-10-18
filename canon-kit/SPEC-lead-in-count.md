# SPEC amendment: lead-in-count

Pairs `list-lead-in-count-unheld`. A spelled cardinal introducing the bullet list below it drifts when the list gains a member. evidence-kit/SPEC.md §The producer-liveness lock read "Two further cases" over three bullets until a close repaired it. `check-manifest-count` reads only cardinals quantifying a declared collection noun, so a sentence-local list is out of its reach. This amendment gives the gate a second arm, which checks such a cardinal against the list instead of banning it.

**Validate, where arm A bans.** The ban's grounds (§check-manifest-count) are the standing cost of a validating gate and an error-prone mapping from total to collection. Neither holds here: the collection is the next block, and its member count is structural. A ban would also red the nearly ninety correct lead-ins the tree carries.

**The arm keys on a cardinal that opens the lead-in sentence.** Probe at spec, a scan of the `CANON_KIT_MANIFEST_FILES` corpus for a paragraph ending in `:` whose next block is a bullet list:

| Rule | Sites | Mismatches |
|---|---|---|
| any spelled cardinal anywhere in the lead-in sentence | 211 | about 30 |
| a cardinal opening the sentence, after an optional `The` | 92 | 3 |

About fifteen of the first rule's mismatches are cardinals quantifying something other than the list ("the two relative spellings are not interchangeable:" over four reasons; "over-selects in two ways and under-selects in one:"). Under the second rule all three mismatches can be judged by hand, and they are listed under point 6.

**Dated posts are outside the arm.** A lead-in count narrates the list as it was written, and a path in `CANON_KIT_TEMPORAL_EXEMPT_PATHS` takes no narration governance (§check-manifest-temporal). The one post mismatch, `docs/posts/2026-07-26-checkwright-v0-16-0.md:37` ("Two limits" over two supported paths and two limits), is that case.

## What changes

### (1) `check-manifest-count` gains arm B, a lead-in count {design-bearing} {user-facing: list-lead-in-count-unheld's Deliverable, "`check-manifest-count` widened … to red a cardinal in a sentence ending on `:` whose next block is a bullet list"; narrowed at spec to a cardinal opening that sentence, and to a count that disagrees, on the probe above}

Replacement for `canon-kit/SPEC.md` §check-manifest-count, its opening invariant sentence. **Not yet applied.**

> Invariant, two arms over manifest prose outside an exempt site: (A) no bare cardinal quantifies a governed collection noun, and (B) a cardinal opening a list's lead-in agrees with the list.

A new paragraph after the measured-marker paragraph, **not yet applied**:

> **Arm B reads a lead-in: a paragraph whose text ends in `:` and whose next block, past at most one blank line, is a bullet or numbered list.** Its last sentence, after a leading list marker, emphasis markers and an optional `The` are dropped, must open with a cardinal in the count adapter's grammar, outside inline code. The cardinal is compared with the list's **items**, the lines opening a `-`, `*`, `+` or `<digits>.` marker at the first item's indent. Deeper lines are nested content, and the list ends at the first non-blank line indented no deeper than an item that opens none. A disagreement is red. The arm validates where arm A bans, because the collection is the adjacent list and its count is structural, so the ban's mapping cost never arises. Arm A's exempt sites hold arm B. A file matching `CANON_KIT_TEMPORAL_EXEMPT_PATHS` is outside it alone, since a lead-in count narrates the list as written (§check-manifest-temporal). **Honest limit:** a cardinal past the opening word quantifies something other than the list as often as the list itself, so it is not read, and a lead-in such as "The trigger over-selects in two ways and under-selects in one" stays unchecked.

`native/src/gates/manifest_count.rs`: the sink keeps the last flushed paragraph and its lines, so the next block is judged at the following line or flush. A finding reads `  <file>:<line>  lead-in count: '<cardinal>' over <n> item(s)`. A second help line follows arm A's: `help: a lead-in count disagreeing with its list — make the cardinal match, or drop it ('The cases:')`. The clean line adds the lead-ins read. The path test is `TemporalValve::path_exempt` in `native/src/gates/manifest_temporal.rs`, built over the path knob alone, so the member declares no section knob.

### (2) The arm's fixtures and declared reads {mechanical}

- `canon-kit/gate-tests/check-manifest-count/bad/` gains a lead-in whose opening cardinal disagrees with its list. `good/` gains:
  - a matching lead-in;
  - one whose list carries nested items and a blank line between items;
  - a mid-sentence cardinal over a wrong count (unread);
  - an exempted mismatch;
  - a mismatch in a file under a temporal-exempt glob set in the fixture's knob file.
- `native/src/gates/mod.rs`, the `check-manifest-count` row: `CANON_KIT_TEMPORAL_EXEMPT_PATHS` joins its declared knobs.
- `canon-kit/checks/check-manifest-count.gate`: `knob:CANON_KIT_TEMPORAL_EXEMPT_PATHS` joins `couples=`, and the `# spec:` gloss names both arms.
- `canon-kit/README.md`'s gate roster line names both arms.

### (3) §check-manifest-temporal states the path valve's new reach {mechanical}

Replacement for `canon-kit/SPEC.md` §check-manifest-temporal, the paragraph opening "The same valves admit a retired path", its first sentence. **Not yet applied.**

> The same valves admit a retired path under §check-docs-cmd assertion C and a retired knob under its assertion B, and the path valve also holds a file out of §check-manifest-count's arm B, so one exemption clears every gate it reaches, and this section, which owns the valves, states their reach once.

### (4) The two in-tree mismatches are reworded {mechanical}

- `canon-kit/SPEC.md:203` (§Layout and configuration), "The three `_MAX` ceilings each take a non-negative integer:" over six knobs, three of them not ceilings. It becomes "The threshold knobs, each with its value set:".
- `lifecycle-kit/SPEC.md:673` (§bin/enter-stage.sh), "The four:" over three bullets whose first names two knobs. It becomes "They are:", since the sentence before it already states the count.

## Producers and consumers

- **Arm B's finding (delta 1).** Producer: the manifest walk the gate already runs, with no new enabling config, so the arm is live wherever the gate is registered, this repo included. Consumer: the committing session through the output contract, on the pre-commit hook, `run-gates.sh` and CI, plus `--run-gate-tests` through the pair. Fields: file, line and cardinal, each named by the help line's remedy; the item count is the evidence.
- **`CANON_KIT_TEMPORAL_EXEMPT_PATHS` as a second gate's input (deltas 1 to 3).** Producer: unchanged. This repo sets `docs/posts/*`. Roster-holding readers: the member's declared-knob row and its descriptor's `couples=` (delta 2), which `check-reads-couples` holds to the reads, and §check-manifest-temporal's reach sentence (delta 3). The knob has no §Layout and configuration bullet; its one statement is that section's valve list.
- **Point 5:** no delta narrows a corpus. The path valve holds a file out of a new arm, not out of arm A, whose corpus is unchanged.
- **Point 6 — each current lead-in.** The 92 lead-ins the probe found satisfy arm B at their current counts, except the three listed above. The two in the governed set take delta 4's rewordings, after which neither opens with a cardinal. The post sits under the temporal-exempt path. The probe, the scan described above over `CLAUDE.md`, `README.md`, the root documents, `*/SPEC.md`, `*/README.md`, `reserve/*/README.md`, `companion/*/README.md`, `doctrine-kit/DOCTRINE.md`, `docs/*.md`, `docs/*/index.md` and `docs/posts/*.md`, is re-run by build as the gate itself.

## Existing sections updated

The roster came from `git grep -n "check-manifest-count\|TEMPORAL_EXEMPT_PATHS" -- canon-kit native/src/gates scripts ':!*/gate-tests/*'` over the tracked tree, plus the probe's two governed mismatches.

- `canon-kit/SPEC.md` §check-manifest-count (delta 1).
- `native/src/gates/manifest_count.rs` (delta 1).
- `canon-kit/gate-tests/check-manifest-count/` (delta 2).
- `native/src/gates/mod.rs`, the member's row (delta 2).
- `canon-kit/checks/check-manifest-count.gate` (delta 2).
- `canon-kit/README.md`, the gate roster (delta 2).
- `canon-kit/SPEC.md` §check-manifest-temporal (delta 3).
- `canon-kit/SPEC.md` §Layout and configuration, the prose-tell threshold lead-in (delta 4).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the declared-roster lead-in (delta 4).
- `.workflow/release-declarations.md`, `## New and tightened gates`: `check-manifest-count` tightened by arm B (delta 1).
- `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md` and `docs/lifecycle-kit/SPEC.md`, the generated mirrors (all deltas).
- `docs/check-graph.html`, regenerated for the descriptor's `couples=` (delta 2).

## Retired spellings

- None — arm A's wording, help line and clean line keep their spellings; delta 4's rewordings retire prose, not a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`) once the iteration's last canon-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Native build and suites** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-gate-tests canon-kit/gate-tests` (with `check-manifest-count.test.sh`) and the full battery run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
