# SPEC amendment: pipe-suites

`check-pipe-membership` reads `walk::tracked_shell_tree`, whose filter drops every `*.test.sh`, so the bespoke unit tests sit outside the gate although nearly all of them set `pipefail`. One of the five membership sites fixed before the gate existed was a suite (`gate-sdk/gate-tests/lib-gate.test.sh`), so the class is attested in exactly the files the corpus leaves out.

**The ruling: the corpus gains the suites the fixture runner runs, and nothing else under a tests dir.** Dropping the `*.test.sh` filter alone reaches none of them. Every tracked suite sits in a `gate-tests/` directory, and `gate-tests` is a member of `GATE_SDK_PRUNE_DIRS`' default, so the walk prunes it before the filter is ever asked. Un-pruning `gate-tests` is refused: it would admit every fixture case tree, whose `bad/` cases carry deliberate violations — this gate's own among them. **A suite is a tracked `*.test.sh` whose parent directory is named `gate-tests` and whose path above that directory is unpruned.** That is the shape §run-gate-tests runs, `<tests-dir>/*.test.sh`, read off the tracked name rather than through `registry::fixture_suites()`: the name rule needs no knob read beyond the prune pair the member already declares, it reaches the gates directory's suites as well as each kit's, and a fixture case can exercise it, since a case's paths are case-relative and carry no pruned component of their own. A file inside a case directory has a case directory as its parent, or a pruned `gate-tests` above it, and stays out. The directory name is the kit convention both `registry::fixture_suites()` and §check-test-hermetic already key on, not a consumer path.

**Measured at authoring (2026-09-23).**

- `git ls-files '*.test.sh'` lists 115 files: 108 suites, each a direct child of a `gate-tests/` directory, and 7 inside fixture case trees (`check-kit-roots-dialect`'s four and `check-test-hermetic`'s three). All 108 suites sit under a pruned `gate-tests` component.
- The widened gate lands green. The 108 suites were copied under non-`.test.sh` names into a scratch repository and the gate was run there (`checkwright-gates --scratch-run` over a copy script): `PIPE-MEMBERSHIP: clean (108 shell file(s) scanned, 108 under pipefail; 4 pipeline(s) into a short-circuiting reader, none fed by a set)`.

## What changes

### (1) The corpus unions the tracked suites {design-bearing}

**Not yet applied.** `native/src/walk.rs` gains a sibling of `tracked_shell_tree`, the tracked suite set by the rule above, read from the same `git ls-files` so an untracked suite stays out on the corpus's existing tracked-only rule. `native/src/gates/pipe_membership.rs` scans `tracked_shell_tree()` followed by it. The two halves are disjoint by construction, since the first drops every `*.test.sh`. `walk::tracked_shell_tree` itself does not change: `check-path-dialect`, `check-gate-substrate-parity` assertion G, `check-gate-exemption-tasks` and `port-blockers --tree` read it, none of them is asked about here, and §port-blockers' sentence calling it "the crate's one tracked-shell-tree rule" stays true.

The registry row is unchanged: the suite rule reads only the prune pair it already declares. So is the descriptor: `kit:*.sh` and `knob:GATE_SDK_GATES_DIR/*.sh` already cover `<kit>/gate-tests/x.test.sh` and `<gates-dir>/gate-tests/x.test.sh`, because a couples token's `*` crosses `/` (§Reading a `couples=` field's reach), and the trigger stays `trigger=*`.

The clean line counts the suites apart, `N shell file(s) and S suite(s) scanned`, so an empty suite half reads as a counted zero rather than vanishing into one total.

### (2) The fixture pair holds the suite half {design-bearing}

**Not yet applied.** The `bad/` case gains `tree/gate-tests/membership.test.sh`, a suite carrying a set-fed membership pipe under `pipefail`, and its `expect.txt` count rises by one. The `good/` case gains `tree/gate-tests/clean.test.sh`, a suite under `pipefail` with the loop form, and `tree/gate-tests/check-x/bad/violating.test.sh`, a set-fed pipe inside a case-shaped directory that must stay unread — the half of the rule a corpus widening can get wrong silently. `good/expect.txt` moves with the new counts. Neither file enters another gate's corpus from the real tree: each sits under this case's own pruned `gate-tests` component, `check-test-hermetic` reads only direct children of a kit's `gate-tests/`, and `check-shellcheck`'s derived set reaches no `gate-tests/`.

### (3) §check-pipe-membership states the widened corpus {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §check-pipe-membership, the paragraph opening "**The corpus** is `walk::tracked_shell_tree`" becomes:

> **The corpus** is `walk::tracked_shell_tree`, the corpus and prune set §check-path-dialect walks, plus the tracked unit-test suites — each `*.test.sh` whose parent directory is named `gate-tests` and whose path above it is unpruned, the `<tests-dir>/*.test.sh` shape §run-gate-tests runs — read on the same code-and-comment split. The suites join because a suite is a shell file under `pipefail` like any other and the class was attested in one; a case directory's files stay out, since its `bad/` trees carry deliberate violations. The clean line counts the two halves apart. **A file is in scope** when a code line sets `pipefail`: `set -o pipefail`, or a combined flag word ending in `o` (`set -euo pipefail`). A line whose code half ends in `\` or `|` continues onto the next, so a pipe broken across lines is one pipeline.

In the **Honest limit** paragraph, the sentence "`*.test.sh` suites are outside the corpus by the corpus rule above." is deleted. In the fixture paragraph (**Fail-closed.**), "in `bad/`, with the count asserted" becomes "in `bad/`, a suite among them, with the count asserted", and the `good/` list gains "a violating file nested inside a case directory under a tests dir".

## Producers and consumers

- **The suite set** (delta 1). Producer: the new `walk.rs` enumerator over `git ls-files` and the prune pair, which every tree running the battery resolves, so no configuration is new. Consumer: this gate's scan alone, at every commit (`trigger=*`).
- **The suite count on the clean line** (delta 1). Reader: `good/expect.txt` (delta 2). `git grep -n "PIPE-MEMBERSHIP: clean"` finds no reader outside the module and the pair.
- **Point 5.** The corpus widens, so the verdict can only gain findings; the gate reds on a violation and never on finding none, asserts no count on a live tree and holds no floor. The `bad/` case's asserted count is the one exact-count reader, and delta 2 moves it.
- **Point 6.** Not reached. No member of the corpus owes a value.

## Existing sections updated

Roster from `git grep -n "tracked_shell_tree\|test.sh" gate-sdk/SPEC.md` and `git grep -n "check-pipe-membership" -- ':!docs'`, run 2026-09-23.

- `gate-sdk/SPEC.md` §check-pipe-membership, the corpus paragraph, the honest limit and the fixture sentence (delta 3).
- `native/src/walk.rs`, the suite enumerator beside `tracked_shell_tree` (delta 1).
- `native/src/gates/pipe_membership.rs`, the corpus and the clean line (delta 1).
- `gate-sdk/gate-tests/check-pipe-membership/`, both cases (delta 2).
- `.workflow/release-declarations.md`, a Tightened gates bullet: `check-pipe-membership` now reads each tests dir's `*.test.sh` suites, so a set-fed membership pipe in a suite reds (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no delta retires a spelling; the removed honest-limit sentence has no second site.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the widened corpus.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The corpus paragraph is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `pipe-membership-corpus-omits-test-suites` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
