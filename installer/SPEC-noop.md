# SPEC amendment: noop

`init`'s stated contract — that a run **makes one commit** containing what it
vendored — is falsifiable today: a run can rewrite three generated projections
and commit none of them, while reporting that it committed. The operator ruled
the semantics on 2026-08-19: **(a) no-op-run-still-commits** — a run `init`
considers a no-op still commits what it rewrote, so "nothing to change" no longer
implies no commit. What (a) obliges is that the contract stop being falsifiable
by an idempotent second run, **not** that the early exit disappear. Its two
rivals stay on the record because the ruling is a choice among them and a reader
is owed what was refused: **don't-regenerate-unmoved**, which needs a freshness
comparison `init` does not do; and **adopter-regenerated**, which changes the
ownership model by taking the three projections out of the manifest.

**A repair was attempted and falsified against the real hop** (landed and
reverted the same evening). Moving the generated-projection block ahead of the
carry-forward loop did not help: the claim helper was not refusing these paths,
and staging order relative to the carry-forward was not the mechanism.

## The mechanism, found at this stage and **not** the one the queue entry names

The entry states the decisive observation as *`init` regenerates all three
projections on every run and only then decides the run was a no-op*. That is an
accurate description of the **symptom**. It is not what keeps the paths out of
the commit, and the difference decides what the fix is.

The carry-forward loop tests membership by piping the written-path list into a
quiet grep. That grep exits on its first match while the writer is still
writing, the writer takes `SIGPIPE`, and under the `pipefail` option the file
sets, the **pipeline's** status becomes the signal rather than the grep's zero.
So the short-circuit that should skip an already-written path does not fire, and
a path `init` rewrote seconds earlier is recorded as **carried at its prior
hash**. Carried paths are excluded from the staged set by construction, so they
are never staged. The fault is **self-amplifying**: each carried path is appended
to the written list, pushing every later lookup further from the tail and deeper
into the same regime.

**Reproduced twice rather than reasoned** — once in isolation and once at the
real starter-profile scale with this tree's own path set, where the carry rate is
over ninety-five percent and all three projections come back carried. It explains
both facts the reorder theory could not: the reorder changed nothing because the
fault is **order-insensitive**, and the hop's commit touched the lock file alone
because the staged set was nearly empty. The same idiom appears a second time, in
the seam-membership probe, where it is harmless only by luck.

**Corrected at build against the running suite: there are two faults, not one,
and the second *is* the ordering the entry named.** The membership fault above
is real and is exactly as described. It is not sufficient. The carry-forward
loop ran **before** the generated projections were recorded, so a prior-manifest
entry for one of them was not yet in the written set when the loop tested it: the
loop carried it at its superseded hash, and the later record could not undo that,
because a carried path is excluded from the staged set. With the membership test
repaired and nothing else changed, the consumer smoke's upgrade hop still failed
on a dirty worktree — which is how this was found rather than argued.

So each half explains the other's failure. The reorder alone could not work while
the membership test was broken: the loop would re-carry the moved-up paths
anyway, which is why it was landed, falsified and reverted. The membership fix
alone cannot work either, because the loop still runs ahead of the only write
path that records after it. **"The fault is order-insensitive" holds of the
membership fault and not of the defect**, and the sentence above is scoped to the
former rather than deleted, because the reorder's falsification is a real result
and its explanation is this.

The invariant the fix restores is the one the loop's own prose already states:
it selects *the paths no `copy_in` visited **this run***, so it must run after
every write path rather than merely late among them. This is mechanics inside the
envelope, not a move toward rival (b): it changes the statement order within one
run and changes nothing about whether or when `init` regenerates.

**Confirmed against the source, not carried on the reproduction alone.** The
authoring stage's account was checked at the next stage directly against both
occurrences in the vendoring verb — the carry-forward loop's short-circuit and
the seam-membership probe — including the reading that makes the second one
harmless: its false negative re-records the seam with no hash, so the path stays
out of the carried set and only the roster count inflates. The claim is therefore
verified at two independent stages, and a build session need not re-derive it.

**This does not reverse the ruling.** (a) remains implementable and this
amendment implements it. It does change what the *minimal* correct change is, and
it opens one question the envelope below sends upward rather than absorbing.

## The envelope

**Asserted.** The membership test is repaired at both sites; `init` commits what
it rewrote; the contract prose stops being falsifiable; the vacuity tripwire
lands with the fix; the baseline row promotes.

**Not asserted.** No change to what `init` regenerates, to when it regenerates,
or to the manifest's ownership model — those are the two refused rivals. No
change to packing or to the version stamp. The early exit **stays**, and delta 2
states why it must.

**The fork the authoring stage parked here is closed.** It asked whether (a)
could be read as *satisfied by construction* once the membership test is
repaired — defensible, since with the projections actually staged a run that
rewrote them can no longer take the early exit at all, but a re-scope of a
recorded ruling and so the operator's to make. **Ruled 2026-08-19: (a) as ruled,
on top of the real fix.** Belt and braces — the defect is repaired at its cause
*and* the no-op exit gains the commit arm. The arm is expected to be inert under
the repair, and that is the point of it: it holds the "makes one commit" contract
over the staged set failing to cover what `init` rewrote, rather than trusting
that it never will again. A build session finding it unreachable therefore lands
it anyway; the fork is not reopened by that finding.

## What changes

### 1. The membership test is repaired, and it is a **prerequisite** rather than part of the fix

The written set becomes an associative array maintained where paths are
recorded, and membership is tested against that array rather than through a
pipeline. This also repairs the second site, and it removes the duplication that
inflates the "files checked" figure the no-op report prints — measured at roughly
twice the true roster today, which is why an operator reading that line saw a
number that could not be right. *design-bearing.*

**It lands in front of (a), and the ordering is not stylistic.** Until the
projections are in the staged set there is nothing for a commit arm to commit, so
(a) landed on top of the unrepaired test would change the report and leave the
worktree exactly as dirty. *mechanical.*

### 2. (a) is implemented **inside** the early exit, because every other shape is wrong

Probed rather than assumed: a commit with an empty index **exits non-zero** and
prints that there is nothing to commit. `init` treats a failed commit as a hard
install failure with a message telling the adopter the files are staged and to
commit them, so moving the commit ahead of the no-op predicate turns the pure
idempotent path into a false, fatal error. The predicate therefore **stays** the
guard on whether a commit is attempted, and (a) can only change what happens on
the branch where the guard says nothing is staged. *design-bearing.*

**The arm carries its own guard, and it is not the index again.** Inside that
branch the index is by definition empty of changes, so an unconditional commit
there reproduces the empty-index failure the paragraph above rules out — one
level in. The arm therefore asks the question the branch has *not* asked: does
any path `init` wrote this run still differ from `HEAD` in the worktree? That set
is the staged set, so the question is answerable without widening what `init`
considers its own, and a negative answer — the expected one under delta 1 — costs
one `git diff` and leaves the idempotent path exactly as it was. *mechanical.*

**Changing the predicate instead — comparing the worktree rather than the index —
is the shape that reaches the reported symptom without fixing the cause**, and it
would leave the stale carried hashes in place. Named because it is the edit the
symptom description invites. *design-bearing.*

### 3. The contract prose becomes honest, at every surface that states it

The published contract appears on the installer's own page, on the public install
page, in the repo README, in the published package description, and in three
`spec:` comments inside `init` itself — one of which asserts precisely the
behaviour (a) changes. None of these is machine-held against the others: the only
install-page parity contract in the tree is over the toolchain bullets. So they
are updated deliberately and enumerated in §Existing sections updated rather than
left to a grep. *design-bearing.*

**The carry-forward prose is the one that needs no change and must still be
cited.** It states that a path is carried *because nothing visited it on the
run*, and that the carried hash is the one `init` wrote. Both are correct as
specification and both are what the defect violates — so delta 1 **restores**
that prose rather than amending it, and the amendment says so, because a reader
comparing the section to the old behaviour would otherwise conclude the prose had
drifted. *design-bearing.*

### 4. The vacuity tripwire, recovered rather than re-invented

Its shape was settled before this stage and drafted once already, then reverted
with the falsified repair. It is the out-of-scope-count pattern: **assert the
artifact-free hop omitted a non-zero number of members before asserting the
worktree is clean**, so the clean-worktree assertion cannot hold over a hop that
rewrote nothing. It counts the omission-declaring lines in the consumer's own
registry, guarded so an absent file counts zero rather than aborting, and its
failure message **names the re-scope remedy and closes the drop escape** — the
arm is re-scoped onto a profile whose kit set ships a binary-dispatched member
`init` seeds, never dropped. It sits in the upgrade arm, immediately before the
clean-worktree assertion it protects. *mechanical.*

**Why it is owed *with* the fix and not before it**: with the fix absent the arm
fails either way, so the tripwire alone would change the failure message while
leaving the coverage class open. And why it is owed at all: the assertion it
guards passed for years over a hop that rewrote nothing, because the lattice
minimum shipped no binary-dispatched member the installer seeds — so the defect
reached the tree **under a green assertion**. *design-bearing.*

**Its scope is the first hop only, and that is stated rather than left implicit**
— the second hop's clean-worktree assertion carries no tripwire, and widening it
is a separate judgment this fix does not make. *mechanical.*

### 5. The baseline row promotes in the **same commit** the entry closes

The suite currently carries a held-constant red keyed to this unit's slug. The
baseline contract **forbids** a blocking slug when the status is pass and
requires every slug to resolve to a live task, so the row must flip to pass with
the slug **dropped**, in the commit that moves the entry out of the live set —
otherwise the baseline gate reds on slug liveness. Tooling never writes this: a
promotion is a human commit. *mechanical.*

**And the run producing that evidence must be made on a committed tree**, because
both the suite and the packer refuse a dirty repo worktree. The promotion is
therefore evidence-then-commit, not commit-then-evidence. *mechanical.*

### 6. Two consumers the defect also degrades, priced rather than fixed here

`uninstall` removes a manifest path only while its recorded hash still matches,
and `diff` reads the same hashes. With the three projections carried at stale
hashes, both read them as adopter-edited: `uninstall` **keeps and reports** files
`init` wrote, so the reverse-to-the-pre-install-tree property silently degrades
after any re-run. Neither smoke arm sees it, because both run against fresh
installs. *design-bearing.*

**These need no separate fix — delta 1 closes them at the source**, since the
stale hash and the missing staging are the same fault. They are stated because
the amendment would otherwise read as being about a dirty worktree, and the
worktree is the visible half of a manifest that was recording values `init` had
already superseded. Whether the smoke should also assert `uninstall` from a
**re-run** consumer is a coverage question this fix does not answer and does not
foreclose. *design-bearing.*

**The operator refused the widen, 2026-08-19, and the refusal is the ruling here
rather than a scoping preference.** These two consumers are **out of scope**:
they are filed to the gap inbox and no work is scoped toward them. Delta 1 may
well close them as a side effect, since the stale hash and the missing staging
are one fault — but a side effect is reported, never asserted as coverage, so
the filed entries stand on their own and the commit says plainly which of the two
it happened to reach. *mechanical.*

## Producers and consumers

**Producer.** Two, in order. The repaired membership test produces the carried
set, which produces the staged set — so after delta 1 the staged set covers what
`init` actually rewrote and the existing commit call carries it. Then (a)'s new
branch on the no-op exit produces a commit on a run the predicate classifies as
changing nothing. Its enabling configuration is none: `init` is not knob-driven
on this path, and the branch is reachable on any run, in any profile, with no
flag set.

**Consumers**, each named with the transition where it reads:

- **The adopter's git history**, at review time — the property the "one commit"
  contract sells, and the reason the clean-worktree precondition is defensible.
- **`init`'s own clean-worktree precondition, on the *next* run**, and
  `uninstall`'s — this is the adopter-visible harm today: residue `init` itself
  wrote makes the next invocation refuse until the adopter commits or stashes it.
- **The manifest's recorded hashes**, read by `uninstall` when deciding whether a
  path is still `init`'s to remove, and by `diff` when reporting drift (delta 6).
- **The consumer smoke's assertions** — the per-profile idempotent re-run, both
  upgrade hops' clean-worktree checks, the seam arm, and the uninstall arms that
  compare back to the pre-install tree object.
- **The no-op report line**, read by an adopter and by the suite's output checks.

**New names.** Delta 1 mints **one**: an associative array holding the written
set, internal to the vendoring verb, with no reader outside it — and it exists to
*delete* a derivation rather than to add one, replacing a pipeline whose result
was already being computed. Delta 4 mints one shell local and one progress line
inside the smoke, both read by the suite's own output.

**(a) mints at most two, and each has a named reader at a named transition.** The
no-op **report line** must stop implying that nothing was committed; its readers
are the adopter, at the end of a run, and the suite's output greps. And a
**commit-message form** distinct from the ordinary vendoring one, if the branch
takes a distinct one; its reader is the adopter's log. That second one is
**deliberately not minted here**: a distinct form would have to be checked
against this repo's own commit-message patterns, since a consumer with an
artifact runs the commit-message gate against whatever `init` writes, and reusing
the existing form has no such cost. So the branch reuses it, and the amendment
records the refusal rather than leaving it to be discovered.

**No field is added to the manifest, no flag to the command line, no knob
anywhere, and no exit code changes meaning.** Stated explicitly because a fix
about commits invites the assumption that one of them moved.

**The narrowing question does not arise** (canon-kit/SPEC.md §The
causal-completeness check, point 5): nothing here narrows a corpus. Delta 1
*widens* the staged set — strictly more paths reach the commit — and delta 4 adds
an assertion in front of an existing one rather than replacing it. The point is
answered rather than skipped, and the answer is that its precondition is absent.

**The seam.** Nothing crosses it: the installer is not a kit, carries no
`checks/` and no `smoke/`, and this fix reads no rule content, no vocabulary and
no roster. The tripwire counts a line shape the installer itself writes. The
packing path is untouched, and the tripwire **does not ship** — the published
package excludes the smoke directory, so the assertion runs in-tree only against
consumers built from packed payloads.

## Existing sections updated

- **installer/README.md §Preconditions** — the clean-worktree precondition is
  grounded on "makes one commit"; the grounding survives and the sentence gains
  what (a) makes of a no-op run. Owned by delta 3.
- **installer/README.md §What `init` does** — the "one commit" sentence and the
  re-run sentence: "nothing to change" stops implying no commit. Owned by
  delta 3.
- **installer/README.md §The manifest** — **no text change**, cited because
  delta 1 restores the behaviour this section already specifies and an update
  target no delta claims reaches build as an orphan. Owned by delta 3.
- **installer/README.md §The consumer smoke** — the upgrade arm's asserted path
  gains the omission-count clause, mirroring the phrasing the binary-less leg
  already carries. Owned by delta 4.
- **installer/README.md — the uninstall sections** — their own commit contract is
  worded in parallel with `init`'s; (a) desynchronizes the pair, so the parallel
  is either restored or its divergence stated. Owned by delta 3.
- **docs/install.md** — the public restatements of the commit and idempotency
  contract. Nothing gates these against the installer's own page, so they are
  updated deliberately. Owned by delta 3.
- **README.md** — the repo root's one-line description of what `init` does.
  Owned by delta 3.
- **`installer/package.json`'s description** — the published package blurb states
  the same contract and ships to the registry. Owned by delta 3.
- **`installer/lib/init.sh`'s `spec:` comments** — three bind to the README
  sentences above, and the one on the no-op exit asserts precisely what (a)
  changes; the carry-forward's own comment makes the claim delta 1 restores.
  Not spec sections, listed because deltas 1 and 3 own them. Owned by deltas 1
  and 3.
- **`installer/consumer-smoke/run-smoke.sh`'s suite `spec:` header** — it
  enumerates the activation path the suite asserts and gains the tripwire. Owned
  by delta 4.
- **`.workflow/validate-baseline.txt`** — the held-constant red promotes to pass
  with its slug dropped, under evidence-kit's baseline contract. Owned by
  delta 5.
- **`TASK-QUEUE.md`'s `installer-init-noop-regen-conflict` entry** — moves to
  Done in the same commit as the baseline promotion, the deliverable being this
  one fix rather than a corpus. Owned by delta 5.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
