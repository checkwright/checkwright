# SPEC amendment: rule-class

Pairs `content-rule-class`. A member with several rule classes prints the class on its finding line, and the SARIF log today drops it: every result's `ruleId` is the member name. The content checks also print two record shapes the placement rule does not read, a path with no line and a byte offset in a built binary, so each such finding lands at the registry line with an empty fingerprint text and every one of them collapses onto one identity. This amendment surfaces the class as a result property, places the two shapes, and keeps a binary finding's offset out of its identity, under one verdict-contract minor.

**What the tree does today**, each bullet with the probe that settled it:

- `sarif::location` reads only `<path>:<line>:` and `<path>:<line>:<col>:`, and `sarif::result` writes no `properties` (`grep -n 'fn location' -A23 native/src/sarif.rs`; `grep -n 'fn result' -A9 native/src/sarif.rs`).
- Two members print a bracketed upper class token after the location prefix: check-prose-tells' `[A]` to `[G]` and check-prose-bounds' `[A]` and `[B]` (`git grep -nE '"\{\}:\{\}: \[|text: format!\("\[' -- native/src/gates`).
- No member prints `<path>: [<CLASS>]` or `<path>@<n>:` (`git grep -nE '[^:]: \[[A-Z][A-Z0-9_-]*\]' -- 'native/src/gates/*.rs'` finds only check-prose-tells' located form; `git grep -nE ':[^ ]*: \[[A-Z][A-Z0-9_-]*\]' -- '*/checks/*.sh' 'scripts/*.sh'` finds none; `git grep -n '"{}@{}' native/src` finds only `native/src/actions.rs`' internal word form, never a finding line).
- The verdict contract is `1.1.0` (`grep -n 'CONTRACT' native/src/sarif.rs`).

## What changes

### (1) A finding's rule class {design-bearing} {user-facing: verdict-identity-fields' ask C, the adopter input on the entry}

**Not yet applied**: gate-sdk/SPEC.md §Output contract, after the paragraph opening "A gate signals success", gains:

> **A finding line may name its rule class.** Where a member holds several rules, the text after a finding's location opens `[<CLASS>]`, `<CLASS>` matching `[A-Z][A-Z0-9_-]*`, followed by a space or the line's end. The class is the rule's stable id within its member, so a reader keys on it and never on the message after it. A member with one rule omits it, since its name is its rule.

**Not yet applied**: §run-gates' result list, after the bullet on a fail result's identity, gains:

> - **A finding's class.** A placed result whose text opens with a class token (§Output contract) carries it, without the brackets, as `properties.ruleClass`. A result with no token carries no property: its `ruleId` is the whole of its rule.

- **Why a property and not a rule per class.** A SARIF rule is one member here, with the member name as its id (§run-gates), and the contract's major covers that one-to-one mapping. A class-per-rule log would rename every `ruleId` a multi-class member emits, a major change, and a member's class roster is its own and changes with it, so the log's `rules` array would move with every new class.
- **Why the grammar is upper-token.** A lower-case bracket such as a queue tag (`[blocked-by: x]`) opens no class, so only a token a member mints as an id is read as one.

### (2) A path-only finding {design-bearing} {user-facing: the entry's deliverable — a binary or a path name located, not collapsed}

**Not yet applied**: §run-gates' result list, the bullet opening "**A red member's finding line.**", is rephrased to name three shapes, each with its path repo-relative:

> - **A red member's finding line.** Each line of its captured output in one of three shapes is one result, its message the line:
>   - `<path>:<line>:` or `<path>:<line>:<col>:`, `<path>` naming a regular file, placed at that region;
>   - `<path>: [<CLASS>]`, `<path>` naming a regular file or a directory, placed on the artifact with no region;
>   - `<path>@<offset>: [<CLASS>]`, `<offset>` a decimal byte offset and `<path>` naming a regular file, placed at a region carrying `byteOffset` alone.

The two new shapes require the class token: a line such as `error: …` or `note: …` is never read as a path, and no line a member prints today changes its placement (the probes above).

### (3) The identity of a located finding {design-bearing} {user-facing: the adopter input on the entry — an offset is never identity}

**Not yet applied**: §run-gates' bullet on a fail result's identity is rephrased so its text is defined over all three shapes: "A finding-line result's text is its line after the location prefix (`<path>:<line>:[<col>:]`, `<path>:` or `<path>@<offset>:`), with leading ASCII whitespace stripped, so a finding that moves to another line or offset keeps its identity." It gains a sentence: "A member whose record carries a list position puts it after the class token and carries nothing volatile there, so its identity is path, member, class and position."

- **Why `fingerprints` and not the `partialFingerprints` the adopter named.** The adopter's need is an identity built from the path, the rule and the term's list position, never the offset and never the term. The recipe above already gives that: the offset sits in the location prefix, which the text excludes, and the term is never printed (the deny-term records, gate-sdk/SPEC-deny-terms.md delta 2 and gate-sdk/SPEC-binary-strings.md delta 3). A second field would hold the same identity under a second recipe. The section's existing ground for `fingerprints` stands: the tool computes the identity whole.
- **Honest limit.** The ordinal still counts identical texts in one file, so a build that adds an earlier hit of the same term shifts the later ordinals, as §run-gates' limit already states for lines.

### (4) The contract's version and goldens {mechanical}

**Not yet applied**: §run-gates, *The verdict contract's version*: "`1.1.0` today" becomes "`1.2.0` today"; the contract's enumeration gains "a result's rule class" after "a fail result's fingerprint"; a note follows the note for `1.1.0`: "The note for `1.2.0`: a result gained its rule class, and a finding naming a path alone or a byte offset gained its placement." `CONTRACT` in `native/src/sarif.rs` becomes `1.2.0`.

- **Why a minor.** The rule's minor covers an added property and an added result-bearing class. Every line a `1.1.0` member prints keeps its placement and its fingerprint, because both new shapes require a class token no such line opens with (the probes above). A `1.1.0` reader ignores the unknown property, and reads a region with no `startLine`, which the registry-file location of an unknown `--only` name already produces.

**Not yet applied**: *The contract's goldens*: `red.sarif` is described as "a finding-line failure carrying a class, a path-only failure, a byte-offset failure, a registry-line failure and a pass", and `failures.sarif` moves with it. `golden_red` in `native/src/sarif.rs`' tests gains the three member lines, and both goldens take the rendered bytes.

**Not yet applied**: §run-gates' closing **Honest limit** ("a result's location is only as exact as the member's finding line") gains: "A path-only or byte-offset record without a class token lands at the registry line."

## Producers and consumers

- **`properties.ruleClass`.** Producer: `sarif::render`, for every placed result whose text opens a class token, under any `--run` with `GATE_SDK_SARIF_FILE` set. Deployed: this repo's `gates` job, which sets the knob (`.github/workflows/gates.yml`), and the installer's CI action. Live classes from day one: check-prose-tells and check-prose-bounds, both registered here (`grep -n 'prose' scripts/gates.list`), then every content check this iteration adds. Consumers: the outside work engine keying a failure to its rule *(inferred: its reader is out of tree, its need lead-relayed)*; code scanning receives the property and is not known to read it.
- **The path-only and byte-offset placements.** Producer: `sarif::location`, reading the lines check-tree-terms' binary arm, check-path-names and check-binary-strings print (gate-sdk/SPEC-deny-terms.md deltas 2 and 3; gate-sdk/SPEC-binary-strings.md delta 3). Consumer: code scanning's annotation on the artifact, and the outside engine diffing a rerun's failures by fingerprint *(inferred, as above)*.
- **A roster-holding reader of the log's shape:** `--run-consumer-smoke` asserts the major is `1` (`grep -n 'verdictContract' native/src/emit/run_consumer_smoke.rs`), which a minor leaves green. No reader in the tree switches on `startLine`'s presence (`git grep -n 'startLine' -- native/src ':!native/src/sarif.rs'` finds none).

## Existing sections updated

- gate-sdk/SPEC.md §Output contract — the class token (delta 1).
- gate-sdk/SPEC.md §run-gates, the result list — the class property, the three shapes, the identity text (deltas 1, 2 and 3).
- gate-sdk/SPEC.md §run-gates, *The verdict contract's version* and *The contract's goldens* — `1.2.0`, its note, `red.sarif` and `failures.sarif` (delta 4).
- gate-sdk/SPEC.md §run-gates, the closing honest limit (delta 4).
- `native/src/sarif.rs` — `location`, `result`, `CONTRACT` and `golden_red` (deltas 1, 2, 3 and 4).
- `docs/contracts/verdict/v1/red.sarif` and `failures.sarif` (delta 4).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — nothing is renamed or removed: the verdict contract gains a property and two placed shapes, and `1.1.0` survives only as the history its note records.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
