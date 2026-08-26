# SPEC amendment: payload-links

Closes `payload-symlink-unextractable-on-windows`. `scripts/pack-installer.sh`'s
`pack_tracked()` vendors each root through `git archive "$COMMIT" -- "$src" |
tar -x`, and on a native Windows host that pipeline **aborts** on the one tracked
symlink in the tree — `tar: Cannot create symlink to <target>: No such file or
directory`, then `Exiting with failure status` — leaving the path absent and the
kit half-vendored.

**The wider fork the entry posed is settled by the governing spec, not by this
amendment's judgment.** The entry asked "whether `gate-tests/` belongs in the
consumer payload **at all**". gate-sdk/SPEC.md §Consumer payload answers it
already: the `good/`+`bad/` fixture pair is one of the four things that ship, and
the reason is stated there — "the consumer's whole verification oracle once the
source is withheld". A payload that withholds the predicate and also withholds
the fixtures discloses nothing an adopter can check. So the fork collapses and
the narrow repair is the whole of the work; delta 4 records the collapse where a
later reader will look for it.

**Two candidate repairs are falsified rather than weighed, and both are the ones
a reader reaches for first.**

*Resolve the fixture to a live target.* `native/src/gates/tree_terms.rs:125`
filters its corpus with `Path::new(path).is_file()`, which **follows** the link.
A resolvable symlink is therefore **scanned**, not skipped: it would change
`good/expect.txt`'s count and destroy the very arm the fixture exists for. The
link's evidentiary value depends on its being dangling, which is exactly what
Windows cannot extract.

*Make the extractor tolerant.* Every form of this — a different `tar` flag, a
`git checkout-index` substitution, a best-effort re-materialisation — is a claim
about Git-for-Windows or Windows `tar` behaviour that **cannot be verified from
this tree**, on this machine, or by any oracle this repo owns. The evidence for
the failure came from one CI run; a repair resting on unverified behaviour of the
same layer would be a guess shipped as a fix. Removing the dependency on symlink
support is provable without a Windows host, and that asymmetry is why it wins.

## What changes

### (1) The tracked symlink leaves the corpus and its assertion moves lane, not away

`gate-sdk/gate-tests/check-tree-terms/good/tree/dangling-link` — the **only**
tracked symlink in the tree, confirmed by `git ls-files -s` filtered to mode
`120000` — is deleted. The arm it carried is rebuilt in
`gate-sdk/gate-tests/check-tree-terms.test.sh`, which already stands up a
`mktemp -d` sandbox repository and already composes banned shapes rather than
spelling them. The new arm creates a dangling symlink there, `git add`s it,
runs the gate, and asserts **green** with the link absent from the scanned
count — the same "proved by greenness rather than by absence" claim, made by a
link constructed at run time instead of one carried in the shipped corpus.
**{design-bearing}**

**No `expect.txt` moves.** `good/`'s tree holds six tracked paths and its
expectation is `2 tracked file(s) scanned`: the two `msg-patterns*` files are
self-exempt, `worktrees/` is pruned, and the symlink was already excluded by
`is_file()`. Deleting it leaves the same two files and the same sentence. The
pair's arithmetic is unchanged, which is what makes this a relocation rather than
a reduction.

**The arm skips-and-declares where `ln -s` fails.** Creating a symlink on Windows
needs a privilege an ordinary account may not hold, so the arm reports a skip
with its reason rather than failing — the honest shape for an assertion whose
*precondition* is a platform capability, and the precise reason the corpus could
not keep carrying it.

### (2) `pack_tracked()` refuses a symlink it is about to fail to extract

`scripts/pack-installer.sh`'s `pack_tracked()` gains a pre-flight before the
pipeline: `git ls-files -s -- "$src"` filtered to mode `120000`. A non-empty
result is **exit 2** naming each offending path, the platform class it breaks,
and the remedy — the payload must not carry a symlink, because the tracked-set
copy is reproduced by `tar` on hosts that cannot create one. **{design-bearing}**

**Fail-closed at the producer is the whole enforcement, and it is exact.** It
covers precisely what is packed — every kit root plus `installer/`, since both go
through this one helper — with no corpus knob to configure and no roster to
maintain. It is exercised on every path that assembles a payload: the release
pack, `installer/consumer-smoke/run-smoke.sh`, and the `installer_smoke` validate
suite, which drives `pack-installer.sh` through all four of its call sites every
iteration.

**Today's failure is worse than a refusal and that is the argument for one.** The
pipeline's status is `tar`'s, so the run reports failure *after* writing a
partial kit; a pre-flight refusal writes nothing and names the cause. The
difference matters most on the host that cannot debug it.

### (3) The fixture-pair form of this assertion is impossible, and the impossibility is recorded

gate-sdk/SPEC.md gains the finding as a stated non-target: **no `good/`+`bad/`
gate can hold "the packed set carries no tracked symlink"**, because a `bad/`
case proving the red would have to *be* a tracked symlink inside a kit root —
reintroducing the exact artifact the invariant forbids, and shipping it to every
adopter in the payload. `GATE_PRUNE_DIRS` excluding `gate-tests` does not rescue
it: pruning removes the path from the gate's *corpus* and not from the *payload*,
so the fixture would still break the extraction it exists to prevent.
**{design-bearing}**

Written down because gate-sdk/SPEC.md §Fixture-pair discipline makes the pair
mandatory for a shipped gate, so the next reader meeting this gap will read the
absence of a gate as an omission and try to close it. It is not an omission: the
four-contract shape is what refuses the gate here, and the producer-side refusal
in delta 2 is the form the invariant can take. This is the same class of
disclosure gate-sdk/SPEC.md §check-tree-terms already makes when it records which
arms a case dir is structurally unable to reach.

### (4) §Consumer payload states that the fixture pair's shipping is what forbids an unshippable fixture

The four-exclusions list in gate-sdk/SPEC.md §Consumer payload gains one clause
on its fixture-pair bullet: because the pair ships, a fixture is **payload
content**, so it is bound by what the payload's transport can carry, and a
fixture that cannot be vendored onto a supported host is a broken fixture however
well it proves its arm. **{design-bearing}**

This is the general rule delta 1's specific removal is an instance of, and it is
the sentence that makes delta 3's impossibility follow rather than surprise. It
also answers the entry's fork on the surface that owns it: `gate-tests/` ships,
and the price of shipping is that it is held to the transport's terms.

## Producers and consumers

**No new state, event, message or field is introduced by any delta.** Deltas 1,
3 and 4 delete a file, relocate an executable assertion, and add prose; delta 2
adds a refusal path to an existing function. The causal-completeness points are
therefore discharged as follows rather than skipped.

**The refusal (delta 2)** is the one new behaviour.

- *Producer:* `scripts/pack-installer.sh`'s `pack_tracked()`, on every call,
  before the `git archive | tar` pipeline. **Enabling config actually set:** none
  — it reads the repository's own index through `git ls-files -s`, which is
  already the helper's context (`$COMMIT` is resolved above it), so the arm is
  live on the next `pack-installer.sh` run in this tree rather than test-only.
- *Consumer:* the invoking session or job, through exit 2 and stderr. Its four
  reachable callers are named in the delta and each already treats a non-zero
  `pack_tracked` as fatal (`pack_tracked … || exit 2` at `:113` and `:121`), so
  no caller needs teaching a new status.
- *Named reader for every field:* the refusal carries one datum, the offending
  path list, and its reader is the same session at the same transition. Nothing
  is written to disk, so no surface acquires an unread field.

**The relocated assertion (delta 1)** has a producer and a consumer that both
already exist: `gate-sdk/bin/run-gate-tests.sh` runs `<tests-dir>/*.test.sh` and
requires exit 0, in this tree through the `gate_sdk` fixture suite and in an
adopter's tree through the same runner over the vendored copy. That the
`.test.sh` **ships** is what makes this a relocation rather than a loss of
consumer-side coverage, and it is why the arm moves there rather than into a
crate unit test, which does not ship (gate-sdk/SPEC.md §Consumer payload
withholds the implementation source, and a unit test lives in it).

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 1 **narrows a corpus** — it deletes a tracked file from the set
`check-tree-terms` walks and from the set the payload carries — so each reader is
enumerated by its **red condition**, not its subject. This is the point's own
attested trap: "a narrower corpus can only remove violations" is false.

- `check-tree-terms` over `good/` — reds when a scanned file matches a banned
  pattern, and its `expect.txt` asserts an **exact count** in prose
  (`2 tracked file(s) scanned`). **Not monotone**: an exact-count assertion moves
  in either direction. Cleared **by arithmetic, not by inspection**: the deleted
  path was excluded by `is_file()` and never counted, so the count is 2 before
  and after. This one must be re-run rather than reasoned at merge.
- `check-gate-fixture-coverage` — reds when a registered gate lacks a `good/` or
  `bad/` case. **Holds a minimum, so not monotone.** Cleared by inspection: the
  deletion removes one file from inside `good/`, never the case directory, so
  both cases still exist.
- `check-tree-terms` over the **live tree** — reds on a tracked file matching a
  banned pattern. **Not monotone in general** (it is a find-something reader, so
  removal cannot add a violation) and cleared here: `gate-tests/` is pruned from
  its corpus in this direction, which gate-sdk/SPEC.md §check-tree-terms states
  explicitly, so the deleted path was never scanned as live-tree content.
- `check-gate-substrate-parity` / the pair-driven runner — red when a case's
  observed output differs from `expect.txt`. **Not monotone** (exact match).
  Covered by the arithmetic above and by re-running the pair.
- `check-workflow-tiering`, `check-core-files` — red on a governed path that is
  neither tracked nor ignored, and on a pinned repo-meta file absent from the
  index. **Monotone and cleared by inspection**: the deleted path is neither a
  `.workflow/` member nor pinned repo-meta.
- `installer_smoke` (the validate suite) — reds on the recorded baseline verdict
  moving. **Not monotone**: its whole verdict is one scenario baselined at
  `fail`, so a *recovery* is as visible as a regression and neither is a red
  today. Named because delta 2 adds a refusal on that suite's own execution path;
  it is the reader most likely to be surprised, and the amendment's own oracle
  that the refusal does not fire on a clean tree.

## Existing sections updated

- `gate-sdk/SPEC.md` §Consumer payload — the fixture-pair exclusion bullet gains
  the transport clause, and the entry's wider fork is answered where the bound
  lives rather than in the queue (deltas 3 and 4).
- `gate-sdk/SPEC.md` §check-tree-terms — the paragraph recording that the pair
  "was widened first in three directions" names the dangling symlink as one of
  them; it is rewritten to record that the symlink arm now lives in the bespoke
  test and **why** — this is the residue paragraph that already owns what a case
  dir cannot reach, so the reason lands beside its siblings (deltas 1 and 3).
- `gate-sdk/SPEC.md` §Fixture-pair discipline — the stated non-target: an
  invariant whose `bad/` case would itself be unshippable payload content cannot
  take the pair form, and takes a producer-side refusal instead (delta 3).
- `gate-sdk/SPEC.md` §Consumer payload, the pack-side paragraph — `pack_tracked`
  is fail-closed on unvendorable content, stated where the one-payload rule
  already describes what assembly may and may not do (delta 2).
- `gate-sdk/gate-tests/check-tree-terms.test.sh` — the file's own header comment
  enumerates the arms a pair cannot reach; the symlink arm joins that
  enumeration with its ground, or the next reader moves it back (delta 1).
- `installer/README.md` §The consumer smoke — no content change; re-read at merge
  to confirm nothing there asserts that `pack-installer.sh` cannot fail before
  cargo, since delta 2 adds an earlier refusal to a path that section describes.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged **at the iteration** rather
      than at this commit, a sibling gate-sdk amendment being in flight.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. `dangling-link` is a path, not a name, so the
      grep is over the path and over the prose that describes the fixture.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The tree carries no tracked symlink** — `git ls-files -s` filtered to
      mode `120000` returns empty, run at build rather than cited from here.
- [ ] **The relocated arm is run, not asserted** — `bash
      gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks` green,
      and the pair's `2 tracked file(s) scanned` re-observed rather than reasoned.
