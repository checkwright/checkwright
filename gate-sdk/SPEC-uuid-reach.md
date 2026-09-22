# SPEC amendment: uuid-reach

The session-reference pattern `^[A-Za-z][A-Za-z-]*: .*<uuid>` bans a trailer-shaped `Key: ` line carrying a full UUID. Its `.*` was bounded by a wrapped line, and on a tree that keeps markdown unwrapped it is bounded by a whole paragraph. The ordinary edit it then reds is a paragraph opening with a `Note: ` or `Session: ` lead-in that quotes an example UUID later on.

**The ruling: bound the value by the token, not by a count.** A harness trailer's value is one token, a bare UUID or a URL ending in one, while prose that quotes a UUID puts words and spaces between the lead-in and it. So the value becomes `[^ ]*`, a run with no space, and the pattern carries no number to choose. The filed candidate, a `[^.;]{0,40}` reach, was run and fails both ways. It still reds the prose shape the entry names, since `Note: the harness prints a session id such as <uuid>` has no `.` or `;` inside forty code points. And it misses a URL-valued trailer, whose host carries a `.`.

**The seam question the entry raised resolves without a knob.** The pattern file is already consumer config: `msg-patterns.list` is copied from the kit's template into the consumer's gates directory and edited there (gate-sdk/SPEC.md §Layout and configuration). So the template's line is the kit's shipped default and a consumer's copy is that consumer's choice, and removing the line is the consumer's off. Both copies take the new shape. The template takes it because an unwrapped tree is a layout the kits support (canon-kit/SPEC.md §check-md-unwrapped), so the paragraph-bounded `.*` is wrong for the kit's own recommended layout and not only for this tree.

**Measured at authoring (2026-09-22).**

- `git grep -n -F '[A-Za-z-]*: .*[0-9a-f]{8}'` finds **six** sites, not the five filed: the template and this repo's copy (line 21 of each), three fixture copies, and a string literal in the unit test `the_shipped_pattern_shapes_compile_and_select_the_right_lines` in `native/src/gates/commit_msg.rs`.
- That unit test's copy of the account-identification pattern has **drifted** from the template: the test spells `` [^A-Za-z`-][^`]{0,6} `` where the template spells `` [^A-Za-z`.;-][^`.;]{0,6} ``, so the test named for the shipped shapes no longer tests them.
- `check-tree-terms` over the tracked tree, with this repo's pattern file carrying the new line (`GATE_SDK_MSG_PATTERN_FILES` pointed at a scratch copy), is clean. A negative control pattern through the same knob reds, so the knob reached the gate.
- Through the engine (`checkwright-gates check-commit-msg <file> <patterns>`): the new line reds `Session-Id: <uuid>`, a URL-valued trailer and a backticked UUID after a lead-in. It passes the prose line above, `check-commit-msg/good/msg.txt` and `check-tree-terms/good/tree/anchored.txt`, and still reds `check-commit-msg/bad/msg.txt`.

## What changes

### (1) The pattern's value is one token, in the template, this repo's copy and the fixtures {mechanical}

**Not yet applied.** In each of `gate-sdk/templates/msg-patterns.list`, `scripts/msg-patterns.list`, `gate-sdk/gate-tests/check-commit-msg/good/patterns.list`, `gate-sdk/gate-tests/check-commit-msg/bad/patterns.list` and `gate-sdk/gate-tests/check-tree-terms/good/patterns.list`, the line

`^[A-Za-z][A-Za-z-]*: .*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}`

becomes

`^[A-Za-z][A-Za-z-]*: [^ ]*[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}`

In the template and this repo's copy, the comment's "a trailer-shaped 'Key: ' line carrying a full UUID" becomes "a trailer-shaped 'Key: ' lead-in whose value, one token with no space, ends in a full UUID". The `check-tree-terms/good/tree/` corpus gains a line pairing a `Note: ` lead-in with a UUID later in its sentence, so the case holds the shape this change stops redding.

### (2) The shipped-shapes unit test reads the shipped template {mechanical}

**Not yet applied.** `the_shipped_pattern_shapes_compile_and_select_the_right_lines` in `native/src/gates/commit_msg.rs` stops carrying pattern literals and reads the pattern lines of `gate-sdk/templates/msg-patterns.list` at test time, through the same comment-and-blank filter the gate applies, so a template edit is what it tests. That removes the drifted account copy as well as the session copy. The template is kit content, not a consumer's list, so reading it keeps the rule §check-tree-terms states against compiling a consumer's live pattern into the crate. The test gains two assertions: a URL-valued trailer hits, and a `Note: ` line quoting a UUID after a space does not.

### (3) §check-commit-msg describes the bound {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-commit-msg, the parenthetical's "a trailer-shaped `Key: ` line carrying a full UUID, the shape of a harness-injected session-id trailer" becomes "a trailer-shaped `Key: ` lead-in whose value is one token ending in a full UUID, the shape of a harness-injected session-id trailer". After the sentence ending "…the account-identification class ruled below is the fourth)", add:

> The value is bounded by the token rather than by a count: a trailer's value is one token and prose quoting a UUID puts spaces before it, so the pattern needs no reach to choose and matches the same on a wrapped or an unwrapped tree.

## Producers and consumers

- **The new pattern line** (delta 1). Producer: the kit template, copied by a consumer, and this repo's tracked copy. Consumers: `check-commit-msg` at the commit-msg hook and `check-tree-terms` at pre-commit, both through `GATE_SDK_MSG_PATTERN_FILES`; `bin/build-native.sh`'s artifact scan reads the same set (gate-sdk/SPEC.md §build-native).
- **Point 5.** The value class narrows from any run to a run with no space, and both readers red on a *match*, so narrowing can only remove reds. The one reader that reds on a *missing* match is the `bad/` fixture, whose `Session-Id: <uuid>` line still matches with an empty value; the engine run above shows it.
- **Point 6.** Not reached.

## Existing sections updated

Roster from `git grep -n -F '[A-Za-z-]*: .*[0-9a-f]{8}'` and `git grep -n -i "uuid" gate-sdk/SPEC.md`, run 2026-09-22.

- `gate-sdk/templates/msg-patterns.list`, the line and its comment (delta 1).
- `scripts/msg-patterns.list`, the line and its comment (delta 1).
- `gate-sdk/gate-tests/check-commit-msg/good/patterns.list` (delta 1).
- `gate-sdk/gate-tests/check-commit-msg/bad/patterns.list` (delta 1).
- `gate-sdk/gate-tests/check-tree-terms/good/patterns.list` and its `tree/` corpus (delta 1).
- `native/src/gates/commit_msg.rs`, the shipped-shapes unit test (delta 2).
- `gate-sdk/SPEC.md` §check-commit-msg (delta 3).
- `.workflow/release-declarations.md`, a Behavior changes bullet: the template's session-reference line changed, so an adopter's copied `msg-patterns.list` re-takes it (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- `[A-Za-z-]*: .*[0-9a-f]{8}` — the paragraph-bounded value (deltas 1 and 2).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the new line.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The §check-commit-msg sentence is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `msg-uuid-reach-unbounded` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
