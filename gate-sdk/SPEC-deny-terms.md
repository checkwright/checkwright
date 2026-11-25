# SPEC amendment: deny-terms

Pairs `adopter-content-checks` (text) and `deny-term-names` (names). A consumer keeps one private list of literal terms its tracked tree must never carry: project names, an account, a host. Today the kit reads only ERE pattern files, and check-tree-terms prints the matched line, so a red run in a CI log republishes the term it caught. This amendment adds the literal-term list with a per-entry rule id, redacts check-tree-terms' record to a location and a rule id, lets that gate read the term list beside its patterns, and adds check-path-names over the tracked path set. check-binary-strings reads the same list (gate-sdk/SPEC.md §check-binary-strings).

**What the tree does today**, each bullet with the probe that settled it:

- check-tree-terms prints `path:lineno:line`, and a path alone for a binary (gate-sdk/SPEC.md §check-tree-terms; `grep -n 'format!' native/src/gates/tree_terms.rs`).
- Every pattern source is an ERE file, one `grep -E` line each with `#` lines ignored (gate-sdk/SPEC.md §check-commit-msg, "Patterns come from"); no knob names a literal-term list (`grep -n 'TERMS' native/src/knobs/gate_sdk.rs` finds none).
- No gate reads tracked path names against a deny set (`git grep -ln 'path component' -- native/src/gates` finds none).

## What changes

### (1) The deny-term list {design-bearing} {user-facing: the adopter input on `adopter-content-checks` — one owner file feeds text, strings and names}

**Applied** (merged into gate-sdk/SPEC.md §Layout and configuration): gate-sdk/SPEC.md §Layout and configuration's rule rosters gain:

> - `GATE_SDK_DENY_TERMS` (default `<gates-dir>/deny-terms.local.list`): space-separated literal-term lists, each **optional**, read by check-tree-terms, check-path-names and check-binary-strings. An absent file contributes nothing; a present unreadable one is exit 3.

**Applied** (merged into gate-sdk/SPEC.md §The deny-term list): a new section, §The deny-term list, after §check-tree-terms:

> **A deny-term list holds literal terms, never patterns.** Each live line is one term, trimmed of surrounding blanks; a blank line or one opening `#` is skipped. A line `# rule: <ID>`, `<ID>` matching the class grammar (§Output contract), names the rule id of every term after it until the next such line; a term before the first takes the reading gate's default id. A malformed `# rule:` line, one whose id fails the grammar, is exit 2, naming the file and line and never the id's neighbours.
>
> **A term's position** is its 1-based ordinal among the live terms of the lists in knob order. A record that must tell two terms apart under one id prints the position, never the term (§run-gates, a fail result's identity).
>
> **A finding never prints a term, a line holding one or the list's contents.** The list exists because its terms are private, and a verdict is published wherever a CI log is.
>
> **The seam ruling.** Kit mechanism is the grammar, the matchers and the knob; consumer config is every term and every id. The kit ships no starter list, since a generic term is no one's secret. A list is gitignored by its default name; one a consumer tracks is self-exempt from every gate reading it, as a resolved pattern file is from check-tree-terms.

- **Why literals beside the ERE files.** A term is the unit the adopter keeps, and three gates match it in three ways (a word in text, a token in a binary, a stem in a path). One ERE could not serve all three, and literals let each gate own its match.
- **Why the id rides a comment line.** Every existing reader of a pattern file already skips `#` lines, so the same `# rule:` line in an ERE pattern file (delta 2) is invisible to check-commit-msg.

### (2) check-tree-terms' redacted record and term arm {design-bearing} {user-facing: the entry's deliverable — path:line and a rule id, never a term}

**Applied** (merged into gate-sdk/SPEC.md §check-tree-terms): §check-tree-terms is rephrased as follows.

- Its record becomes `<path>:<line>: [<ID>]`, one per matching line per distinct id, ordered by line then id, and a binary's `<path>: [<ID>]`. The second ruling bullet ("A banned shape inside a tracked binary yields a path-only record") keeps its ground and takes the new shape, which §run-gates now places.
- `<ID>` is the `# rule:` id in force above the matching pattern or term, else `TEXT-DENIED`. A `# rule:` line in an ERE pattern file follows §The deny-term list's grammar.
- It reads `GATE_SDK_DENY_TERMS` beside its pattern files: a term hits a line where it occurs with no ASCII letter or digit on either side, ASCII case folded. Term files join the self-exempt set. The member's `# armed-by: content` line gains `GATE_SDK_DENY_TERMS`.
- The paragraph "The pair carries every arm of the derivation" is rephrased: `bad/` gains a term hit and a line hit by two ids, and its `expect.txt` lines carry no matched text; `good/` gains a term inside a longer word (`term` inside `terms2`), which does not hit.

- **Why the line goes for ERE hits too.** A private pattern file is a deny list in another grammar, and printing the matched line names what it matched. A person fixing the hit opens the line the record names.
- **Why check-commit-msg keeps its record.** It judges a message on the committing machine and prints into that terminal, never into a CI log.

### (3) check-path-names {design-bearing} {user-facing: the adopter input on `deny-term-names`}

**Not yet applied**: a new section, §check-path-names, after §The deny-term list:

> `checks/check-path-names.gate` (`precommit`, binary-dispatched). Invariant: no tracked path has a component carrying a deny term as its stem. Its corpus is `git ls-files` under the shared prune dirs, each path split on `/` into file and directory names; identifiers inside a file are never read. A component hits a term when, ASCII case folded, it equals the term or opens with the term followed by `-`, `_` or `.`: `foo` hits `foo`, `foo-cli`, `foo_x`, `foo.rs` and `foo.d`, never `food` or `foobar.rs`.
>
> The record is `<path>: [<ID>]`, `<path>` the shortest prefix of a tracked path whose last component hits, printed once however many paths lie under it, `<ID>` the term's id else `NAMES-DENIED`. Two terms of one id on one prefix give one record. It takes no valve: a name is renamed, never excused.
>
> Its terms are `GATE_SDK_DENY_TERMS` (§The deny-term list). With no live term it prints `not applicable: no deny term configured` and exits 0, and declares `# install: on-surface` and `# armed-by: content GATE_SDK_DENY_TERMS`. Outside a work tree it refuses at exit 2; an unreadable list is exit 3. Born native, no shell form; criterion 4 clears, its corpus being path names, never a descriptor's content.
>
> **Honest limit.** A manifest's package or binary name is outside the corpus.
>
> The pair: `bad/` holds a directory hit with two files under it (one record), a file hit by `-`, one by `_` and one by `.`, and a mixed-case hit; `good/` holds components the term only prefixes (`food`, `foobar.rs`) and a term inside a component (`xfoo`). The knob file in each case points `GATE_SDK_DENY_TERMS` at a case-local list.

- **Why the shortest prefix.** A directory named for a term holds every file under it, and one record per file would bury the one rename that fixes them all.
- **Why `.` is a boundary.** A file named for a term carries the term as its stem before an extension, so `foo.rs` and `foo.d` name the term as plainly as `foo-cli` does; `foobar.rs` stays clean, its stem being `foobar`.

## Producers and consumers

- **The deny-term list.** Producer: the consumer's own file, named by `GATE_SDK_DENY_TERMS`. Deployed: the orchestrator adopter's private list *(inferred: its configuration is out of tree, lead-relayed)*. This repo binds no list, so the term readers are not applicable here and `doctor` names them disarmed (gate-sdk/SPEC.md §The install disposition). Consumers: check-tree-terms (delta 2), check-path-names (delta 3), check-binary-strings (gate-sdk/SPEC.md §check-binary-strings), and `doctor` through the `armed-by: content` lines.
- **The `# rule:` id.** Producer: the consumer's list or pattern file. Consumer: each reading gate's record, then `sarif::render`'s `ruleClass` (gate-sdk/SPEC.md §run-gates).
- **The redacted record.** Producer: check-tree-terms, registered here (`grep -n tree-terms scripts/gates.list`) and live over this repo's `msg-patterns.list`. Consumers: the committing session, CI's SARIF upload and the outside engine *(inferred, as above)*. A roster-holding reader: the pair's `expect.txt` and `check-tree-terms.test.sh`'s record-order case, both rewritten (delta 2). `bin/build-native.sh` reads the same pattern files to verify an artifact and prints its own record (gate-sdk/SPEC.md §build-native); it is unaffected.
- **check-path-names.** Producer: the battery, registered in this repo's `scripts/gates.list`. Consumer: the committing session and the SARIF log, placed by the path-only shape. Roster readers of a new member: `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity` and `gates::REGISTRY`.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration, rule rosters — `GATE_SDK_DENY_TERMS` (delta 1).
- gate-sdk/SPEC.md §The deny-term list, new (delta 1).
- gate-sdk/SPEC.md §check-tree-terms — record, ids, term arm, pair (delta 2).
- gate-sdk/SPEC.md §check-path-names, new (delta 3).
- `native/src/knobs/gate_sdk.rs` — the knob row (delta 1).
- `native/src/gates/tree_terms.rs`, its `.gate` armed-by line, its pair and `check-tree-terms.test.sh` (delta 2).
- `native/src/gates/path_names.rs`, `checks/check-path-names.gate`, its pair, `gates::REGISTRY`, gate-sdk/README.md's roster block and `scripts/gates.list` (delta 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no name is retired: check-tree-terms keeps its name and knobs, and its record's `:line` text suffix is a shape, not a spelling a reader greps for.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
