# SPEC amendment: init-claim-stickiness

The spec pass TRAJECTORY.md's 2026-08-04 ruling requires **before any code**:
"the spec pass on installer/README.md §The manifest comes before any code",
because the fix "changes what a recorded hash *means* for a file `init` did not
write that run, which is a contract change rather than a repair, and settling it
in code first would be settling it by implementation."

`installer/` has no `SPEC.md`; `installer/README.md` is its canonical spec (every
`# spec:` comment under `installer/lib/` resolves there), so that file is this
amendment's merge target.

## What changes

### 1. `files` means *what `init` last wrote*, not *what `init` wrote this run* {design-bearing}

The manifest's `files` map is `init`'s **ownership roster**. A recorded hash is
the content **this installer last wrote at that path**, on whichever run last
wrote it — not the content of the tree at the end of the current run, and not the
content the current payload would have written.

Today the two readings coincide for every path `init` actually writes, and they
diverge for exactly one class: a path `claim()` refused because the adopter
edited it. `copy_in` returns before `record`, the guarded-seed re-claim loop
skips `under_kit` paths, and the path leaves `files` entirely — so the *next* run
reads its absence as "never installed", claims it, and overwrites the adopter's
work with no report. The protection §init promises unconditionally survives one
upgrade and then inverts.

The invariant this restores, stated positively: **a path leaves `files` only when
`init` stops shipping it.** An adopter edit changes who may write the path; it
never changes whether `init` is tracking it.

### 2. `record()` takes an optional hash; the claim-refusal branch supplies it {design-bearing}

`record()` today appends a path to `WRITTEN`, and `manifest()` hashes the **tree**
at emit time (`init.sh:336`, `lock_hash "$ROOT/$f"`). So the naive fix — calling
`record` on the refused path — writes the *adopter's* hash into `files`, which
tells the next run that `init` wrote the adopter's content. That run finds
`cur == want`, claims the path, and overwrites it silently. **The naive fix does
not remove the defect; it moves it one upgrade further out.** This delta exists
because of that, and the amendment records it so build does not re-derive it.

The interface: `record <path> [hash]`. With one argument it keeps today's
behavior (hash the tree at emit). With two, the given hash is emitted verbatim.
`copy_in`'s refusal branch supplies `prior_hash "$2"` — the hash already on
record — so the entry is **carried forward unchanged** rather than recomputed.

*Why the prior hash and not the hash the current payload would have written.*
Both protect the file on the next run, so protection does not discriminate. The
**revert** case does. An adopter who restores the file to the content `init` put
there has handed the path back; `claim()` should succeed and `init` should resume
owning it. That happens only when the recorded hash is what `init` actually wrote.
Recording what the payload *would* have written would report the path as changed
forever, and would record a write that never happened.

### 3. §The manifest's agreement invariant is narrowed to the state that holds it {design-bearing}

`installer/README.md:339-343` grounds `artifact`'s separate key on "the invariant
the consumer smoke asserts over every entry in it" — every `files` entry agreeing
with the tree. Under delta 1 that invariant is **false in general**: an
adopter-edited path is deliberately recorded at a hash the tree no longer has.

It stays true where the smoke actually asserts it — immediately after an `init`
into a tree carrying no adopter edits — so the smoke assertion does not move. What
moves is the prose, which currently states as unconditional an invariant that is
conditional. The one-hash-family-per-map argument for `artifact` is untouched and
keeps its force: it never depended on tree agreement, only on the hash *function*
being uniform across the map.

### 4. No schema bump, and the compatibility argument for it {design-bearing}

`files` keeps its shape (`path` → `git hash-object` hash), so
`CHECKWRIGHT_LOCK_SCHEMA` stays `checkwright-lock v1`. An older reader meeting a
manifest written under the new meaning finds an entry whose hash does not match
the tree, and does exactly what it does today: reports it changed and leaves it
alone. The old reader's behavior on the new data is the behavior the new contract
wants, so there is nothing for a version key to protect.

### 5. The second-upgrade smoke arm {mechanical}

`installer/consumer-smoke/run-smoke.sh`'s upgrade arm reaches exactly one hop
(v1 → v2) and asserts the case that is already correct. It is extended to chain a
third packed version onto the same consumer, with no fresh adopter edit — the
point is that the already-edited, already-reported file is still protected on the
*next* hop — asserting both that the content is still the adopter's and that the
run still reports it. Against today's code that arm goes red, which is what makes
it the reproduction rather than a restatement.

`gate-sdk/bin/upgrade-smoke.sh` is **not** where this lives and is named here only
to rule it out: it vendors by wholesale directory replace and never drives
`claim()`/`record()` or `checkwright.lock` at all, so a reader must not read "there
is already an upgrade smoke" as coverage of this path.

### 6. A bounded residual, filed rather than folded in {mechanical}

A path the **payload stops shipping** also leaves `files`, and if a later payload
re-adds it the same silent overwrite is possible. It is a different cause — there,
`init` relinquished the path deliberately, which is the roster behaving correctly —
and folding it in would widen this unit past the ruling that admitted it. It is
filed to the gap inbox for close to disposition, per the gap-disposition rule.

## Producers and consumers

No new manifest field is introduced, so the "every new field has a named reader"
point binds nothing new. What changes is one existing field's meaning and one
internal interface, surveyed across the whole installer tree — every reader of
`.files` found by grepping `installer/` and the repo outside it:

**New interface — `record <path> [hash]`**

- **Producer** — `copy_in()`'s claim-refusal branch (`installer/lib/init.sh:181-185`),
  reached on every run where `claim()` returns 1. That branch is live in the
  shipped surface today (it is the branch the existing upgrade arm exercises), so
  the producer needs no enabling config and is not test-only.
- **Consumer** — `manifest()` (`init.sh:321-345`), which emits `WRITTEN` into
  `files`. It is the sole consumer; `WRITTEN` has no other reader.

**Changed meaning — a `files[path]` hash.** Every reader, and what each does with
the change:

- **`init`**, via `prior_hash()` (`init.sh:157-162`) and `claim()` (`init.sh:166-176`).
  Reads the hash. This is the reader the change exists for: absence of a key stops
  being overloaded between "never installed" and "installed, then dropped from
  tracking", and only the first survives.
- **`doctor`**, via `lock_own_file()` (`installer/lib/common/lock.sh:31-39`), called
  at `doctor.sh:90` and `doctor.sh:109`. Reads the **key set only**, never the hash
  — it disambiguates the consumer's real seam file from same-named fixtures inside
  vendored kit trees, and already filters out every `<kit>/*` path. So its logic is
  unaffected in both the under-kit case (filtered either way) and the non-kit case
  (a path that previously dropped out now persists, which is the roster being more
  complete, not differently shaped).
- **`installer/consumer-smoke/run-smoke.sh:111-127`** — the manifest-agreement check.
  Reads every entry's hash and requires tree agreement. It runs against a
  **freshly initialized** consumer, where no adopter edit exists, so it keeps
  passing unchanged — delta 3 narrows the prose to match where the assertion
  actually stands, rather than moving the assertion.
- **`uninstall` — named, and honestly unbuilt.** There is no `uninstall` verb in
  the tree: `installer/lib/` holds `init.sh` and `doctor.sh` only, and the verb
  roster is derived from that directory (`installer/bin/checkwright.sh:18-23`). The
  queue entry and TRAJECTORY.md both reason about `uninstall` reading this roster;
  the only trace in the tree is a comment at `init.sh:290` anticipating it. It is
  committed work rather than speculation — `installer-lifecycle-verbs`, rung 4 of
  TRAJECTORY.md §PRIORITY DIRECTIVE — so this amendment states the contract it will
  inherit and **designs none of it**: a complete roster is what an uninstall needs,
  and delta 1 is what makes the roster complete. No delta here waits on it.

**Nothing outside `installer/` reads the manifest** — `checkwright.lock` and
`.files` appear nowhere else in the tree (repo-wide grep, no path filtered).

## Existing sections updated

Each target names the delta that owns it, so build adopts no orphan:

- **`installer/README.md` §The manifest** — the `files` row of the field table
  (delta 1), the hash-family paragraph's agreement claim (delta 3), and the
  no-schema-bump note (delta 4). This is the section the operator ruling names,
  and it is updated **before** any code delta lands.
- **`installer/README.md` §init** — the "Re-running is idempotent and
  non-destructive" promise (README.md:87-96) currently reads as unconditional and
  is, today, false on the second upgrade. Delta 1 makes the prose true rather than
  weakening it; the sentence needs no hedge once the roster is sticky.
- **`installer/README.md` §The consumer smoke** — the arm roster gains the
  second-upgrade hop (delta 5).
- **`installer/lib/init.sh`** — `record()`, `copy_in()` (deltas 1-2). The
  `# spec:` comment at `init.sh:339` cites §The manifest and stays correct.
- **`installer/consumer-smoke/run-smoke.sh`** — the upgrade arm (delta 5).

No governed *name* is added anywhere: `record`'s second parameter is internal to
`init.sh`, and no new knob, file, tag or config surface appears. The provenance
seam is not approached — nothing here is rule content, and `installer/` ships no
kit literal.

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
      deferred). Delta 6's residual is filed at authoring time, not at merge.
