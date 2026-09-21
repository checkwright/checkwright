# SPEC amendment: cwd-anchor

§The path-dialect contract obliges a shell script that composes two roots to anchor
its own cwd first, and nothing asserts it. §check-path-dialect names the hole and
leaves open what a red site is supposed to become. **This amendment rules the
satisfying value and adds the arm that asserts it.** A red site becomes an anchored
one: its first act is the contract's first builtin, `cd "$(pwd -P)"`. The oracle is
a third arm of `check-path-dialect` that pairs the two facts the hazard needs.

**The satisfying value was already written, and the open question was a different
primitive.** The contract's idiom block states the obligation as two builtins. The
first is `cd "$(pwd -P)"` before anything is derived from `BASH_SOURCE`. The second
is a re-anchor after a `git rev-parse --show-toplevel` `cd`. A script with no git
producer of its own owes only the first, because nothing re-enters the producer's
spelling afterwards. The open question the section recorded came from
`gate-sdk/lib/test-hermetic.sh:29`, which tests a caller-supplied binary path for
absoluteness from its text. That is the *absoluteness* primitive on a
caller-supplied value, not the cwd anchor. It is separate exposure, outside this
arm, and is filed as its own gap by this spec session. Ruling it here would widen
the unit to every textual absoluteness test in the shell tree (six sites by
`git grep -n -E '== /\*|/\*\)' -- '*.sh'`).

**The measurement (2026-09-21).** Seven non-test tracked shell files derive a root
from `BASH_SOURCE`
(`git grep -l BASH_SOURCE -- '*.sh' | grep -v '\.test\.sh' | grep -v gate-tests`),
and none carries `cd "$(pwd -P)"`. Five of them apply string arithmetic to a root
they derive that way, so an arm keyed on either fact alone reds five to seven files
that compose nothing foreign. Only `gate-sdk/lib/test-hermetic.sh` binds **two**
roots, at `:4` and `:19`, and does arithmetic on them: a suffix strip at `:21` and a
join at `:29`. That is the entry's measured one-of-seven. The probe was a per-file
scan binding every `<name>="$(cd … BASH_SOURCE … pwd)"` capture as a root, then
asking for `$<root>/` or `${<root>%`/`${<root>#` on any bound root.

## What changes

### (1) A cwd-anchor arm on `check-path-dialect`'s shell corpus {design-bearing}

**Not yet applied.** The arm runs over the tracked shell tree the producer arm
already walks, with the same prune set and the same code-and-comment split. It reads
one file at a time, because the contract judges consumption within a file.

- **Roots.** A root is a name bound on one line from a command substitution that
  `cd`s to a path derived from `BASH_SOURCE` and prints `pwd`, or from one that
  captures `pwd` or `pwd -P` after a `git rev-parse --show-toplevel` `cd`. An
  `export`, `local` or `readonly` prefix does not change the binding.
- **Red** when a file binds **two or more** roots, at least one of them from
  `BASH_SOURCE`, and applies string arithmetic to any bound root. String arithmetic
  is `$<root>/` or `${<root>}/` inside a word, or a `${<root>%…}` or `${<root>#…}`
  strip. Nothing else in the file clears it.
- **Clearance.** A line whose code half is exactly `cd "$(pwd -P)"`, before the
  file's first root binding, clears the file. No `spec:` verdict or exempt token
  clears this arm, for the reason the locality arm gives: a clause every site can
  cite its way out of is the review-held clause it replaced.
- **Finding text** names the file, the first root binding's line, and the remedy:
  anchor the cwd with `cd "$(pwd -P)"` before deriving the first root.

The fixture pair gains one case per arm branch. `bad/tree/` gets a two-root file
with a join and no anchor, plus a file anchored *after* its first binding. `good/tree/`
gets the same two-root file anchored first, and a one-root file doing arithmetic,
which stays green by the two-root rule. `bad/expect.txt`'s count line moves from 9
to 11, and that is what gives the new green files a reader.

### (2) `gate-sdk/lib/test-hermetic.sh` takes the anchor {mechanical}

**Not yet applied.** It inserts `cd "$(pwd -P)"` as its first executable line, before
the `:4` binding. The file is sourced as the first act of every `*.test.sh` suite, and
anchoring the sourcing shell's cwd is what the obligation asks. A single-dialect host
sees no change. The oracle is the arm of delta 1 going green on the tree, plus every
fixture suite that sources the library still passing.

### (3) §check-path-dialect and §The path-dialect contract state the arm {mechanical}

**Not yet applied.**

- In §check-path-dialect, replace the paragraph opening **And the cwd-anchor clause
  is unasserted on the shell side** with a paragraph stating the arm of delta 1:
  corpus, roots, the two-root red, the anchor as its sole clearance, and its honest
  limit. The limit is that a file composing one root with a caller's foreign
  absolute path passes. That shape is the absoluteness primitive's, not this arm's.
- In the same section, the opening invariant sentence gains the clause "and every
  shell file composing two roots anchors its cwd first". The **Fail-closed, and
  the honest limit** paragraph's monotonicity claim still holds, since the arm reds
  only on a found violation.
- In §The path-dialect contract, the paragraph after the idiom block gains one
  sentence. It says that a script with no git producer owes the first builtin alone,
  and that `check-path-dialect`'s cwd-anchor arm holds it.

## Producers and consumers

- **The arm (delta 1).** Producer: the `check-path-dialect` run already registered
  in `scripts/gates.list` at `precommit` with `trigger=*`, so every commit reaches it
  with no descriptor change. Consumers: the battery and the hook, which need no
  change because the member's name, tier and couples are unchanged, and the fixture
  runner through the updated pair. No roster-holding reader gains a name: no gate,
  knob, directive or arm is minted.
- **Point 5.** The arm widens the gate, so the monotonicity it states stays true. It
  reds on a found violation, never on finding none, and holds no floor.
- **Point 6.** The corpus the arm obliges is every file that binds two roots. The
  probe above enumerates one member, `gate-sdk/lib/test-hermetic.sh`, and delta 2
  gives its satisfying value.
- **Release declaration.** The gate is zero-config and gets stricter, so build adds a
  `check-path-dialect` bullet to the Tightened-gates section of
  `.workflow/release-declarations.md`. The bullet names the remedy line from the
  finding text (gate-sdk/SPEC.md §upgrade-smoke).

## Existing sections updated

Rosters from `grep -n "cwd-anchor\|anchor its own cwd" gate-sdk/SPEC.md`, the
BASH_SOURCE probe above, and `ls gate-sdk/gate-tests/check-path-dialect/*/`.

- gate-sdk/SPEC.md §check-path-dialect and §The path-dialect contract (delta 3).
- `native/src/gates/path_dialect.rs` and
  `gate-sdk/gate-tests/check-path-dialect/` (delta 1).
- `gate-sdk/lib/test-hermetic.sh` (delta 2).
- `.workflow/release-declarations.md` (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no name is removed; the arm adds a red condition to an existing gate.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the new arm.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The replaced "unasserted" paragraph's
      facts survive: the propagation mechanism and the refusal of a shared normalizer.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `shell-cwd-anchor-clause-has-no-oracle` moves to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Fails closed.** `bad/` reds on the unanchored two-root file and on the late
      anchor, and the count line is asserted.
- [ ] **Measured at landing.** Before delta 2, the arm reds exactly
      `gate-sdk/lib/test-hermetic.sh` on the live tree; after delta 2, it is clean.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
