# SPEC amendment: positional-reference

**A positional reference into a collection is a restated fact about the collection, and the doctrine names only the count.** Derivation-first calls a collection's count the archetypal derivable fact, and `check-manifest-count` gates a bare cardinal. A member's position is derived from the same collection, and it goes stale the same way. README.md once said "The last line, `--run-demo`, is the adoption walkthrough", and then the generated battery-roster block gained a later line. docs/install.md carried two more such sentences against its recipes. All three were rewritten to name the referent. canon-kit/SPEC.md §check-amendment-retired-spelling already records the durable fix for the numbered case: a citation that names its referent rather than its number has nothing left to decay. No doctrine line states it for *first*, *last*, *above* or *below*.

**The rule, not a gate.** A narrow gate keyed on a positional word near a marked or generated block was weighed and is refused. It is recorded as the rule's honest limit, and an audit-roster class carries the cadence.

**Measured at authoring.** The sweep matched `the (last|first|final|next) (line|row|section|entry|bullet|paragraph)`, `the lines? (above|below)`, a bare `above`/`below`, `the following` and `the preceding`. It ran over the tracked governed markdown less fixtures, posts and mirrors, keeping prose lines within three lines of a block.

- Near a `<!-- name:begin -->` marker it found 6 hits. About two are positional citations of a generated block (README.md "The block below is …", gate-sdk/README.md "the full meta-gate roster below"). Both point at the adjacent block, and neither names a position inside it. The rest are "the last stamp" and "below the floor".
- Near a fence it found 13 hits, all describing a neighbouring paragraph or table row.

The referent of a positional word is not decidable from the word, which is the renumber slice's argument (canon-kit/SPEC.md §check-amendment-retired-spelling). A proximity gate at that precision cries wolf (gate-sdk/SPEC.md §When a gate earns its place).

**The digest stays.** Adding "a position" to Derivation-first's digest would change the always-loaded line, and `check-doctrine-registration` holds every adopter's agent-file bullet to it, so every adopter would red until re-running `--install-doctrine`. The body carries the rule where a reader of the rule looks, and the digest's "a roster, a count" already names the collection.

## What changes

### (1) Derivation-first names a member's position beside its count

doctrine-kit/DOCTRINE.md, rule Derivation-first {mechanical}. **Not yet applied.**

The sentence "A count of a collection is the archetypal derivable fact: the collection is its own counting surface, and a stated total anywhere else is a duplicate counting surface, off by one at the next member." becomes:

> A count of a collection is the archetypal derivable fact, and a member's position in it is the next. The collection is its own counting and ordering surface, so a stated total, or a position such as "the last line" of a roster or "rule 19", duplicates it and is off by one at the next member. Name the member instead. A pointer to an adjacent block ("the block below") names no position inside it and is not this case.

The *Enforced by* clause gains a last sentence:

> A position has no gate: its referent is not decidable from the word, the renumber slice [canon-kit/SPEC.md](../canon-kit/SPEC.md) §check-amendment-retired-spelling records, so a consumer audit-roster class is its cadence ([lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §The audit roster).

The digest line is unchanged.

### (2) This repository's audit roster gains the class

`.workflow/audit-roster.txt` {mechanical}: a new block, following the file's grammar and staying under `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`:

```
class: positional-reference
scope: a sentence locating a member by its position inside a collection another surface grows (the last line of a roster, the first row of a generated table, a numbered rule cited by number), on governed prose, per doctrine-kit/DOCTRINE.md Derivation-first; un-gateable, since a positional word's referent is not decidable from the word. Corpus: the lines the iteration's range added to a marked or generated block, a fenced recipe or a numbered list, from git diff <base>..HEAD, each read for prose elsewhere that locates a member by position. A pointer to an adjacent block names no position and is clean. Fix by naming the member.
due: an iteration that adds, removes or reorders a member of a generated block, a fenced recipe or a numbered rule list
last: never
```

## Producers and consumers

- **The doctrine line.**
  - Producer: the vendored DOCTRINE.md, read by a session that loads the doctrine behind CLAUDE.md's pointer.
  - Consumers: the prose author, and the close review through the new roster class.
  - The line adds no field and no knob.
- **The roster class.**
  - Producer: this repository's audit roster.
  - Consumers: the close stage's roster review, which judges due-ness from `due` and `last`, and `check-audit-roster`, which grades the block's grammar. A never-swept class carries `last: never` and stops there.
- **Readers of the doctrine file.**
  - `check-doctrine-registration` reads rule names and digests, and neither moves.
  - `check-docs-mirror-fresh` byte-compares the mirror, which delta 1 regenerates.
  - The upgrade smoke carries the vendored file.
  - `git grep -n DOCTRINE -- native/src` finds the install arm and the registration gate reading rule names and digests (`native/src/doctrine.rs`), `native/src/emit/stage_rules.rs` reading the craft rules' stage lines, and the overhead meter counting the file's bytes. None reads a methodology rule's body.
  - `check-prose-bounds` holds the file under `.workflow/prose-bound-ceiling.txt`. Delta 1's sentences each stay under the 45-word bound this repository sets.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md`, Derivation-first (delta 1).
- `docs/doctrine-kit/DOCTRINE.md`: the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 1).
- `.workflow/audit-roster.txt`: the class (delta 2).
- `.workflow/prose-bound-ceiling.txt`: the `doctrine-kit/DOCTRINE.md` row, re-stamped to the count `check-prose-bounds` prints if delta 1 moves it (delta 1).
- `.workflow/release-declarations.md`, under Behavior changes: "**`doctrine-kit/DOCTRINE.md` Derivation-first** — a member's position inside a collection (the last line of a roster, a rule cited by number) is named beside its count as a derived fact prose restates; name the member. The digest is unchanged. Nothing to do." (delta 1).

The roster came from `git grep -n -e DOCTRINE -e 'Derivation-first' -- native/src .workflow scripts CLAUDE.md`, whose only rule-body reader is the mirror.

## Retired spellings

- None — the amendment rewrites a sentence and adds a roster class, and renames nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the line and the class.
- [ ] **Instruction surfaces: instruction only**: the roster block's scope states the sweep and its fix and carries no grounds.
- [ ] **Merged with no information lost**: Derivation-first reads as one rule; the refused gate's grounds survive as its *Enforced by* sentence.
- [ ] **Amendment deleted**: this file is removed on merge, and `ls doctrine-kit/SPEC-*.md` lists no file of this amendment.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `positional-reference-rule` moves to Done in the landing commit, before the drain stage.
