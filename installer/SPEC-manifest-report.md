# SPEC amendment: the manifest arm reports its own disagreement

The consumer smoke's manifest arm reds on a native Windows runner with
`477 of 477 manifest entries disagree with the tree` and says nothing about why.
Twelve rounds have been spent on that leg; the last four bought nothing because
the arm that fails is not the code that reports, and the bespoke CI diagnostic
standing in for a report has now failed twice for reasons of its own. This
amendment moves the report into the arm.

It buys a **reading**, not a repair and not a green leg. What this iteration
commits to delivering is settled in §Definition of Done below, deliberately and
at authoring time, so close is not left arguing it.

**The seam, ruled: nothing here is kit mechanism, nothing is private rule
content, and nothing becomes consumer config.** Every surface this amendment
touches is this repo's own — `installer/consumer-smoke/run-smoke.sh` is the
acceptance harness, declared `no-port`, carried by neither transport and
received by no adopter; `.github/workflows/gates.yml` is this repo's CI;
`installer/README.md` governs the installer, which is not a kit and must not
become one. No kit gains a gate, a template or a knob. The one place a knob
would have been natural — gating the failure report — is refused in
§Producers and consumers on the ground that a report no deployed configuration
enables is the dead-producer shape, so the report is unconditional on the
failure path and there is no `<KIT>_<KNOB>` to name.

## What changes

### (1) The manifest arm reports its own disagreement, in place, before it fails

`installer/consumer-smoke/run-smoke.sh`'s `assert_install` manifest loop gains a
**failure report**: when the loop ends with `mismatch` non-zero, the arm prints
a bounded, discriminating fact set about the disagreement it found and *then*
fails exactly as it does today. {design-bearing}

The report is on the failure path only. On a green leg the arm prints what it
prints now — the `manifest: N file(s) agree with the tree` line — and the report
costs nothing.

**Why the arm and not a step beside it.** The report has to run while the
disagreeing consumer is still on disk, and `run-smoke.sh` mktemps its scratch
under `trap cleanup EXIT` with `cleanup() { rm -rf "$SCRATCH"; }`, so nothing
after the run can open it. The standing answer to that was a CI step that stands
up a *second* consumer by the same route — a duplicate of the smoke's own
install path, maintained in YAML, unrunnable locally, and free to drift from the
check it diagnoses. It did drift, twice and in two different ways, both recorded
in delta 5. Removing the duplication outranks maintaining it: the arm already
holds the failing state, so the arm is where the report belongs.

**The honest limit, stated rather than gated.** The report is failure-path code
and the only host that executes it is the host that fails, so no green run
exercises it and no fixture pair can. What holds it is shape rather than
coverage: it is a straight-line sequence of prints with no branch that can
change the arm's verdict, and the `fail` that follows is the one that stands
today. A report that cannot alter the verdict is a report a stale line can only
make less useful, never wrong-acting.

### (2) The sample is chosen rather than taken first-come

The report covers **at most two paths**, and neither is chosen by arrival.
{design-bearing}

- The **first** disagreeing path, which is the one a reader would have looked at
  anyway.
- The **artifact's `files` row** — the path §The manifest calls an ordinary
  `files` row recorded with the same `git hash-object` hash every other entry
  carries — whenever the manifest records an `artifact` key and that row is in
  the disagreeing set.

First-come alone is not a sample, it is an accident, and on this leg the
accident has a direction: round 12's first disagreeing path was
`gate-sdk/README.md`, a text file, so a report keyed on it would have shown the
end-of-line-shaped case and never the one that refutes it. The artifact row is
the discriminating case precisely because git classifies its blob binary and
converts a binary blob under no configuration — a hypothesis that survives on a
`.md` and dies on the `.exe` is a hypothesis the report has to be able to kill.

Where the payload carries no artifact — the binary-less leg installs exactly
such a payload — the manifest records no `artifact` key, the second sample is
absent, and the report says so rather than printing a blank.

### (3) Four hash values, and the truth table that reads them

For each sampled path `P` the report prints four values, each labelled and each
rendered **byte-exactly** beside its plain form, together with the standard
error of every call that produced one. {design-bearing}

- **`want`** — `files[P]` out of the consumer's `checkwright.lock`: what `init`
  recorded.
- **`got`** — `git hash-object -- "$C/P"` from the smoke's own current
  directory: the failing read, spelled exactly as the arm spells it.
- **`own`** — `git -C "$C" hash-object -- "P"`: the same command in the
  repository context `lock_hash` runs in, which is the only thing the two
  call sites differ by.
- **`raw`** — `git hash-object --no-filters -- "$C/P"`: the file's bytes with
  the attribute mechanism removed. `--no-filters` ignores attributes entirely,
  so this value does not depend on which repository the call discovers, which is
  what makes it the fixed point the other three are measured against.

Two values cannot separate *one side filtered* from *the bytes changed*; four
can, and the reading rule is stated here rather than left to the next session:

| observation | reading |
| --- | --- |
| `want == own == raw` and `got != raw` | the read side's context applies a filter the write side's does not; the defect is at `run-smoke.sh`'s call site and the two-call-site narrowing is confirmed |
| `want == own == got` and all differ from `raw` | both contexts filter identically, so the hashes agree and the arm could not have failed on this path — the disagreement is in the comparison, not in the hashing |
| `own == got == raw` and `want != raw` | the bytes on disk are not the bytes `init` hashed; the porcelain below and the control in delta 4 say which |
| any of the four is not 40 lowercase hex | the value is not a hash — a stray byte, a truncation, or a refusal; the byte rendering shows which and the captured standard error names the refusal |

The byte rendering is not decoration and it is not optional: a value carrying a
trailing carriage return compares unequal and *prints* equal, and no other line
in the report can see it.

Once per failing profile, and outside the per-path block because each is a fact
about the run rather than about a path, the report also prints:

- `git -C "$C" status --porcelain` and `git -C "$C" log -1 --stat`, truncated —
  the direct witness for the third row of the table. A worktree that has
  diverged from what `init` committed is the one shape that explains a
  disagreement set containing a binary, and it costs one command to see.
- `core.autocrlf`, `core.eol` and `core.safecrlf` **with their origins**, in the
  consumer and in the smoke's own repository — the mechanism behind the first
  row, where a value alone would not say which file set it.
- `git check-attr -a` for each sampled path, in both repositories — the other
  half of that mechanism, since an attribute reaches a path the config does not.

Every one of these has a named reader in the table above or in the paragraph
that introduces it. Nothing is printed for completeness.

### (4) The artifact digest is a filter-free control on the content question

When the manifest records an `artifact` key, the report recomputes the
artifact's SHA-256 from the tree and prints it beside the recorded
`artifact.digest`. {design-bearing}

This costs nothing new — §The manifest already records that digest, and
`installer/lib/common/digest.sh` already owns how this host takes a SHA-256 —
and it settles the content question outright, because SHA-256 over the file is
taken by no git filter and in no repository context. If the recomputed digest
equals the recorded one, the artifact's bytes are exactly the bytes `init`
published; a `files`-row disagreement on that same path is then not a content
change, and it cannot be an end-of-line conversion either, because git converts
no blob it classifies binary. Both surviving content hypotheses die on one line.

The control is stated as a control, so a reader is not free to read a matching
digest as an assertion about any other path: it speaks for the artifact alone.

### (5) The bespoke CI diagnostic step is deleted, not repaired

`.github/workflows/gates.yml`'s `read one manifest disagreement in place` step
is removed. {design-bearing}

It is not being deleted for tidiness. It has failed twice, each time for a
reason the arm cannot have:

- It stands its consumer up with `--profile full`, and the smoke's failing check
  is on **starter**. A diagnostic reproducing a different profile from the one
  that failed is reproducing a different run.
- The full profile is what met `init-vendor-staging-argv-overflow` — `git`
  refusing an over-wide argv while staging the vendored set — so the step bailed
  at its own early guard and printed none of the five things it exists to print.

Deleting the step **does not repair that overflow and must not be read as
repairing it**. `init-vendor-staging-argv-overflow` owns one `git` invocation's
argv width, a native-Windows adopter on the full profile still cannot install,
and that entry stands untouched by this amendment. What deleting the step
removes is this leg's *dependence* on the full profile, not the defect.

The arm inherits the right profile by construction: it runs inside whichever
profile failed, so there is no profile to select and none to get wrong.

### (6) `--no-filters` at the two call sites is refused this iteration, and the refusal is recorded

The obvious move — pass `--no-filters` at `installer/lib/common/lock.sh`'s
`lock_hash` and at `run-smoke.sh`'s manifest arm, so neither call can be
filtered — is **refused as this iteration's work**, and the refusal is written
into §The manifest so the candidate reads as judged rather than as unnoticed.
{design-bearing}

Two grounds, and the second is the stronger:

- **It is a guessed repair, on a leg whose method ruling is cause-read-first.**
  Round 6 spent a round on two guessed repairs that could never have landed,
  because the unguarded command was in the gate and not the generator's
  prologue. A local probe run at this authoring session weakens the guess
  further rather than supporting it: on two throwaway repositories A and B, a
  CRLF-bearing text file and a NUL-bearing binary in B, hashed from B's own
  directory and from A's under `core.autocrlf` in each of `false`, `true` and
  `input`, with and without a `* text=auto` `.gitattributes` in B, **every pair
  agreed**. Attributes are looked up from the file's own directory chain rather
  than from the discovering repository's, which is why. That probe is a Linux
  host's and the leg is Windows', so it demotes the hypothesis and does not
  retire it — but a repair aimed at a mechanism a probe cannot reproduce is a
  guess whatever else it is.
- **It is a change to what a `files` hash means, which no cause read carries the
  authority to make.** §The manifest defines a `files` hash as change detection
  over what `init` wrote. A filtered hash and a raw hash disagree about exactly
  one population — an adopter whose edit is a line-ending change — and switching
  to `--no-filters` decides that `init` now notices such an edit and refuses to
  overwrite it. That is a behavior change to the non-destructive re-run on every
  platform, bought to fix a report on one, and it belongs to a unit that is
  scoped to it.

### (7) What the leg has measured so far is written into the canonical section

The findings this authoring session read off round 12's job log land in §The
consumer smoke rather than dying with this file. {mechanical}

The amendment is deleted at merge, so a measurement whose only home is this file
is a measurement the next rider of the leg re-buys. Four facts qualify:

- All 477 lines read `manifest hash disagrees with the tree`, none read
  `manifest names a file that is not there` — the arm's two branches, and the
  failure is genuinely a hash disagreement rather than a missing path.
- `scripts/checkwright-gates.exe` is in the disagreeing set. Every hypothesis
  that turns on end-of-line conversion has to survive that, and none does.
- The run carries zero `fatal` lines, so a per-path refusal is evidence-against
  rather than open — but the arm never printed `got`, so it is not closed
  either, which is what delta 3's byte rendering settles.
- The host facts: `core.autocrlf false`, `core.symlinks true`,
  `core.longpaths` unset, `core.filemode false`, an empty porcelain on a fresh
  checkout, git 2.55.0.windows.5, bash 5.3.15, and the failing profile is
  `starter` — the profile that vendors one kit and writes a 477-entry manifest.
  The consumer's own battery passed, `All 11 gates passed`, immediately before
  the manifest arm failed.

## Producers and consumers

**The failure report is a new output; it introduces no new state and no new
field on any wire format.** `checkwright.lock`'s schema is untouched, and this
amendment adds no configuration knob — deliberately, because a report gated on a
knob no CI job sets is the dead-producer shape, and the one host that needs the
report is the one nobody is standing at.

- **Producer** — `assert_install` in `installer/consumer-smoke/run-smoke.sh`, on
  the branch where the manifest loop ends with `mismatch` non-zero, immediately
  before the existing `fail`. Reachable with no enabling configuration: the
  branch is the one the leg has taken on every round since round 12, and the
  Linux leg never takes it.
- **Consumer** — the job log of `.github/workflows/gates.yml`'s
  `install smoke on a native Windows host` step, which already pipes the smoke
  through `2>&1 | tee "$log"`, and thereby the session reading that log with
  `gh run view <id> --log`. No second mechanism is introduced and no artifact is
  uploaded: the log is already the leg's published surface, and the leg is
  already `continue-on-error`, so the report reaches its reader on a red run.
- **Reader for every value printed** — delta 3's truth table for the four
  hashes, delta 3's own prose for the porcelain, the config origins and the
  attribute dump, and delta 4's paragraph for the artifact digest. Any value
  this amendment could not name a reading rule for was dropped rather than
  printed.
- **Consumer of the deletion in delta 5** — none. The step's only reader was a
  session reading the log, and the arm now prints what the step was to print.

**Delta 5 narrows a corpus, so each reader's red condition is named rather than
its subject.** The narrowed corpus is `.github/workflows/gates.yml`, which loses
one step, its `run:` body and its comment block. The gates whose corpus it is:

- `check-action-run-shell` — reds on **finding** a `run:` block that is not
  ShellCheck-clean at its dialect. Monotone: removing a block removes at most a
  violation.
- `check-action-pinning` — reds on **finding** a `uses:` ref that is not a
  40-hex SHA or a repo-local path. The removed step carries no `uses:`.
  Monotone.
- `check-action-permissions` — reds on **finding** a job that consumes the token
  without declaring its scopes. The narrowing removes a step, never a job, and
  `install-smoke-windows` keeps its `permissions:` block. Monotone.
- `check-action-gh-repo` — reds on **finding** a job whose `run:` bodies invoke
  `gh` with no repository context. The removed body invokes no `gh`. Monotone.
- `check-comment-tier` — reds on **finding** a non-directive comment. The
  removed block is comment prose, so the narrowing can only remove violations.
  Monotone.

None of the five holds a minimum, a coverage floor or an exact count, so all
five clear by inspection. The battery is still the oracle and build runs it;
this enumeration exists so a build session knows which verdicts it may *not*
clear by inspection, and the answer here is none.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the manifest arm's contract gains
  the failure report: that a disagreeing arm reports before it fails, which two
  paths it samples and why neither is first-come, the four values and the truth
  table that reads them, the once-per-profile facts, the artifact control, and
  the honest limit that the report is failure-path code no green run exercises
  (deltas 1, 2, 3 and 4).
- `installer/README.md` §The consumer smoke — the measured record of what the
  native Windows leg has established so far, so it survives this file's deletion
  (delta 7).
- `installer/README.md` §The manifest — the `files` hash's definition states
  that it is a **filtered** `git hash-object`, and that moving it to
  `--no-filters` is a change to which adopter edits `init` notices rather than a
  spelling fix, with the refusal and its date recorded (delta 6).
- `.github/workflows/gates.yml` — the `read one manifest disagreement in place`
  step and its comment block are removed, and the two reasons it never reported
  are recorded where they are now owed rather than in the deleted comment
  (delta 5).

## Definition of Done

**This iteration delivers a reading, not a green leg and not a repair.** The
host entry names `observation-predicate-entry-cannot-drain-in-its-own-iteration`
as its wedge, so the predicate is settled here and close does not argue it.

- [ ] **The report is landed and reproduces the failing arm by being it** —
      deltas 1 through 4 in `installer/consumer-smoke/run-smoke.sh`, delta 5's
      deletion in the workflow, deltas 6 and 7 merged into `installer/README.md`.
- [ ] **One push is watched to completion and the Windows leg's log is read.**
      The push budget is one push for the cause read plus close's, which is the
      entry's own 2026-08-30 operator ruling and the standing one-to-two cap.
      Widening it is an envelope change and is not this iteration's to take.
- [ ] **The reading is recorded on `platform-support-ci-matrix`**, in a commit
      that rides close's push, in exactly one of two dispositions:
      **cause identified** — the mechanism named, the printed lines that
      establish it quoted, and the repair costed and filed rather than started;
      or **cause not identified** — each hypothesis the run retired named with
      the printed line that retired it, and the surviving set named, so the next
      round narrows instead of re-guessing.
- [ ] **The entry does not complete, and that is the settled outcome rather than
      a shortfall.** Its completion predicate is the leg green with the roster
      join taken, which is a remote-oracle observation the close push cannot
      produce. At close it returns to the deferred section `[design-pending]`
      under its own 2026-08-30 red-cause limb — file and defer without looping —
      and this amendment merges and is deleted in the same commit, which is what
      the bidirectional pairing requires of both halves.

**Why the wedge does not bite this iteration, stated so it is not re-derived.**
The recorded wedge is that an entry whose completion predicate is an observation
of a remote run cannot be drained by the iteration that buys it, because the
drain gate sits upstream of the close push that produces the observation. That
is true of an entry whose observation the *close* push produces. It is not true
here: this iteration's budget is two pushes, the cause-read push is the first,
and its verdict therefore arrives *between* the two — early enough to be read,
recorded as a local commit, and carried out on the second. The discriminator is
which push produces the observation, not whether the entry has one.

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
