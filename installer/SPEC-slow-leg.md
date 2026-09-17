# SPEC amendment: the consumer smoke's slow leg

The native Windows install smoke is the slowest leg of the `gates` workflow, so it
sets the wait on every watched push. This amendment takes the one term a profile
shows is dominant out of it, disposes of the two terms the profile shows are not,
and joins a scratch-exhaustion witness to the upgrade arm so the next silent
upgrade-pack failure on the Intel macOS leg reads from its finished log.

**The profile this design rests on, bought at authoring rather than carried.** No
spawn count existed, so the arms were timed off a finished run's per-line log
timestamps: gates run 35214069775, jobs `install-smoke-windows` and
`install-smoke-macos-intel`. Each `say` line is stamped when printed, so the work
between two lines is attributable to the assertion the second one reports. Totals
per work class, summed over every install the smoke drives:

| work | Windows (526 s smoke step) | Intel macOS (349 s) |
| --- | --- | --- |
| manifest hash loop (one `git hash-object` child per recorded file) | 236.7 s | 79.5 s |
| value arm (two battery runs per profile) | 41.5 s | 56.6 s |
| `init` | 38.0 s | 23.0 s |
| battery | 36.1 s | 61.4 s |
| pack | 22.4 s | 26.4 s |
| idempotent re-run of `init` | 21.4 s | 15.2 s |
| follow-up arm | 11.3 s | 12.2 s |

The loop runs seven times per smoke (four profiles, the download arm, the
toolchain-free arm, the artifact arm) over 492 to 1216 entries, about 33 ms a
child on Windows. That is the entry's term 1, and the filer's unre-run figure of
about 250 s stands. Term 3's figures do not: a full-profile battery is 6.1 s on
Windows, not about 63 s, and a full-profile `init` is 5.7 s, not about 60 s.

## What changes

### (1) The manifest arm hashes the recorded roster in one child

`assert_install`'s manifest loop in `installer/consumer-smoke/run-smoke.sh`
stops spawning one `git hash-object` per entry and hands every hashable path to a
single `git hash-object --stdin-paths`, read back through `read_stream`. {design-bearing}

**The shape.** The loop becomes two passes over the one `files_raw` capture, and
the capture itself is unchanged. The first pass does what the loop does before
the hash today, in the same order: count the line, hold `raw_first`, take the
declared CR strip, split `path` and `want`, and take the missing-file branch. It
collects each surviving entry's path spelled `$C/<path>`, which is the operand the
one-file call hashes today. One `git hash-object --stdin-paths` child then runs
over that list from the same working directory, and `read_stream` reads its
answer. The second pass pairs answer *i* with surviving entry *i* as `got` and
runs the comparison branch unchanged, so the disagreement echoes, `bad_hash`, the
shape test, `raw_bad` and the report's operands all keep their current order and
spelling.

**Why `--stdin-paths` is still the independent second reading.** The entry's
design question was whether batching re-authors that operand. It does not. In
`git hash-object`, a path read from stdin and a path given on argv go through the
same filter lookup. The argv form alone prefixes a relative path with the
subdirectory the command ran in, and the smoke passes absolute paths. Probed at
authoring rather than argued: over nine absolute paths from one working
directory, the per-path loop and one `--stdin-paths` child gave byte-identical
output (git 2.55). What the reading is independent *of* never included git's
hashing implementation, which the one-file form shares with `init` too. It is
independent of `init`'s code, its path list and its working directory, and all
three still differ. `init` records through the same batch form
(§The manifest, *A roster is hashed in one child*). The two call sites stay two
invocations, and the smoke hands git the manifest's key order, not `init`'s copy
order.

**Three refusals keep the batch from reading empty or misaligned.** Each exits 2
through `blocked`: a failed batch or a misaligned answer is a harness
precondition, not a finding about the consumer.
- **A path the batch cannot carry** is one containing a carriage return or line
  feed, or starting with the double quote git unquotes on stdin. It takes the
  one-file call in the first pass, mirroring §The manifest's fallback, so it
  keeps its place in the pairing.
- **A batch that exits non-zero** is refused, naming the exit code and the
  batch's standard error. Probed: one missing path makes the child fatal at exit
  128 and it stops. So the first pass's existence check is what keeps the
  missing-file branch a count rather than a refusal, and it has to run before the
  batch.
- **An answer whose line count differs from the path count** is refused, naming
  both counts. Pairing by index is only sound when the counts are equal, and a
  short answer would otherwise put one entry's `got` beside another's `want`.

**No fallback to one call per file on a failed batch.** That is on purpose, and
it differs from `init`. `init` falls back so that batching is never why an
adopter's hash reads empty. The smoke is the thing that measures the batch, and a
silent fallback would bring back this leg's whole cost with nothing to show why.

### (2) The report's `got` operand and truth table name the batch

The manifest report's `got` stays a held operand, but where it comes from
changes, and `reread` changes from a re-run of the same call to the one control
that separates the batch from the one-file call. {mechanical}

Replacement text for installer/SPEC.md §The consumer smoke. **Not yet applied.**

- The **`got`** bullet becomes: "**`got`** — **held.** The line the arm's single
  `git hash-object --stdin-paths` child answered for `$C/P`, paired with P by
  its position among the entries the loop hashed and carried out of the loop on
  the failure branch. It is the comparison's other operand, not a description of
  one. It is spelled out because applying `want`'s correction to one operand of
  two is exactly how the remaining asymmetry stayed invisible."
- The **`reread`** bullet's first sentence becomes: "a re-read.
  `git hash-object -- "$C/P"`, the one-file call, run at report time from the
  same working directory as the batch." The rest of the bullet is kept.
- The row `got != reread` becomes: "the batch's answer for this path is not what
  the one-file call gives now. Either the value was mangled when the batch was
  captured or read back, or the batch filed another path's answer here. The two
  octet dumps name the byte and its position, and a `got` that is a well-formed
  hash of a different path is the second case."
- The row `want == got`, byte-equal, both held keeps its reading and adds the
  batch as a second pairing: "…so a pairing is wrong. Either `bad_hash`
  associated one entry's `want` with another entry's `got`, or the batch answer
  was paired off by one. Read the sampled path against the loop's own echo order.
  The count refusal rules out a short answer, not a reordered one."

The comment directives in `run-smoke.sh` that describe the capture follow the
same change. The one above the comparison names the batch as `got`'s source, and
the new first-pass and batch lines each carry a `# spec:` pointer to this section.

### (3) A scratch witness in the upgrade arm

`run-smoke.sh` gains `scratch_witness`. It prints the free space on the
filesystem holding `$SCRATCH` (`df -k`) and the space the scratch tree uses
(`du -sk`) as indented `say` lines, and it never changes a verdict. {mechanical}

**Where it runs.** Once before the upgrade arm's first pack, so every run on every
leg records a baseline at the point the Intel macOS failure happened. Again on the
failure branch of each of the two upgrade packs, ahead of `blocked`, so a failed
pack prints the disk state at the moment it failed, next to the empty `PACK_OUT`.

**It never refuses, and it adds nothing to the preflight.** When `df` or `du` is
missing or exits non-zero, the witness says which one and carries on. `df` and
`du` stay out of `SMOKE_TOOLS`, because a diagnostic that could stop the run would
be a new way for the leg it diagnoses to fail. The output is `say` lines, which
the header grammar does not parse (evidence-kit/SPEC.md §Layout and
configuration), so the `installer_smoke` scenario roster is unchanged.

**What it settles, and what it does not.** It tests the joined entry's hypothesis
and nothing more. A failed upgrade pack printed next to near-zero free space
confirms scratch exhaustion. Ample free space rules it out and leaves a regression
or a runner transient. No cause is asserted until a failure has been read against
the witness.

Replacement text for installer/SPEC.md §The consumer smoke, added as the upgrade
arm paragraph's last sentence. **Not yet applied.** "Before its first pack, and on
either pack's failure branch, the arm prints the free space and the scratch
tree's size through a witness that never changes a verdict. A pack that dies
printing nothing is still readable from a finished log: if the scratch
filesystem ran out, the witness shows it."

### (4) The arms stay serial, and the section says why

The two terms the profile shows are not the lever get an answer on the owner
surface instead of staying open. {mechanical}

Replacement text for installer/SPEC.md §The consumer smoke. **Not yet applied.** It
re-phrases the paragraph's closing clause "the suite packs four separate times
across a ~10-minute run", whose count and duration are both stale, and adds one
paragraph after that paragraph:

- Stale clause becomes: "since the suite packs several times over a long run".
- New paragraph: "**The arms run in sequence, and that is part of the contract.**
  The printed arm headers are a parsed scenario roster in print order
  (evidence-kit/SPEC.md §Layout and configuration). `ENTRY` and `RUN_PATH` are
  globals each arm reassigns, and `VALUE_RED` and `REGISTRY` are built up across
  the profile loop. The cross-version reversal arm reads the payload the upgrade
  arm removed a file from. Running arms concurrently would have to buffer every
  arm's output back into header order and pass each of those values between
  processes. On the leg this repository makes binding, that trades a known
  duration for failures that do not reproduce. What the suite spends goes into
  the per-install work each profile repeats. A reduction starts from a per-arm
  profile of a finished run, not from the shape of the arms."

**The grounds for not doing term 2 or term 3, which the passage does not carry.**
Once delta 1 lands, the profile puts the Windows smoke step at about 290 s and the
Intel one at about 280 s. The two legs then roughly tie, and each still spends
80 to 90 s in its crate-build step before the smoke. Concurrency inside the smoke
would save part of the Windows leg's remaining per-profile work, and less
wall-clock time, because the tied Intel leg would still set the watch. The price
is buffered output, shared state passed between processes, and scratch
contention on a binding leg. Term 3 has no lever: the battery and `init` together
account for 74 s of Windows' 526 s across seven installs.

### (5) The two entries' terminal moves

Build moves both entries before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`). {mechanical}

- **`install-smoke-slow-leg-residue` → `## Done`.** All three of its design
  questions are answered: term 1 by delta 1, and terms 2 and 3 by delta 4 and the
  profile above. What the entry delivers is this leg's residue, not one step in a
  series of them.
- **`binding-intel-leg-failed-one-run-in-two` → demoted back to `## Deferred`.**
  The witness does not diagnose the failure. It is what makes a diagnosis possible
  when the failure happens again, so the entry outlives the amendment
  (canon-kit/SPEC.md §Merging an amendment, step 4). It goes back to the position
  it was promoted from, with its `[design-pending]`, `[cost: iteration/high]` and
  `[surface: .github]` tags restored from the promoting commit's diff. Its UNTESTED
  HYPOTHESIS paragraph is compressed to one line saying the witness has landed and
  which output a recurrence is read against. That line keeps the entry inside
  `check-queue-entry-budget`.

## Producers and consumers

- **The batch answer (delta 1).** Producer: `git hash-object --stdin-paths`,
  reached on every `assert_install` call. All seven installs of every
  `installer_smoke` run and every `gates` smoke leg reach it with no knob.
  Consumer: the second pass's comparison, reading the answer through
  `read_stream`. The answer has two readers: the comparison reads each line as
  `got`, and the count refusal reads the line count. When a line carries a CR,
  `read_stream`'s own declaration is the reader of the strip it took, as it
  already is for the other streams.
- **The count and exit refusals (delta 1).** Producer: the batch's exit status and
  the equality between its answer count and its path count. Consumer: `blocked`,
  exit 2, read by the `installer_smoke` suite runner and the workflow step's
  `rc`. Red condition: a non-zero batch, or a count other than the number of
  paths handed in. Neither fires on a run that is green today, because every
  recorded path is a plain vendored kit file.
- **The unbatchable-path fallback (delta 1).** Producer: the first pass's test for
  a CR, an LF or a leading `"`. Consumer: the one-file call, whose answer takes
  that entry's slot. Today no recorded path reaches it, and no configuration in
  the payload makes one. So this producer is dead on every current install, and
  it is kept for the same reason §The manifest keeps the matching fallback:
  batching must never be why a hash reads empty. It is recorded as dead so the
  merging session does not take it for a covered path.
- **The scratch witness (delta 3).** Producer: `scratch_witness`, called on the
  green path before the first upgrade pack and on both upgrade packs' failure
  branches. Consumer: someone reading a finished leg log. No gate reads it, and the
  evidence-kit smoke-log parser skips it by the header grammar. Its fields are
  free space and scratch size, both read against the joined entry's hypothesis in
  delta 3. They have no other reader, so none is added.
- **No corpus narrows (point 5).** No scenario is added or removed, and no glob or
  file set is touched. The `installer_smoke` roster, derived from top-level
  `printf` headers, is the reader such a change would affect. Delta 3 adds
  indented lines only, and delta 1 adds no header.
- **No enumerable corpus is obliged (point 6).** Delta 1's refusals apply to every
  recorded entry, and each entry's satisfying value is the answer the one-file
  call gives. The probe above established that for absolute paths.

## Existing sections updated

- `installer/consumer-smoke/run-smoke.sh` — `assert_install`'s manifest loop
  becomes two passes and a batch, and the comment directives on that loop follow
  (delta 1). `scratch_witness` is added and called at the upgrade arm's three sites
  (delta 3). Roster probe: `git grep -n 'hash-object' installer/consumer-smoke/run-smoke.sh`.
  Line 623 is the only per-entry loop call site. The upgrade and seam arms'
  one-file calls hash two fixed files each and stay as they are.
- `installer/SPEC.md` §The consumer smoke — the `got` and `reread` bullets and the
  two truth-table rows (delta 2), the upgrade-arm witness sentence (delta 3), and
  the stale pack-count clause plus the serial-arms paragraph (delta 4). Probe:
  `git grep -n 'got="\$(git hash-object\|capture time inside the loop\|four separate times' -- installer/SPEC.md`.
- `docs/installer/SPEC.md` — the generated on-site mirror of the section above,
  regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in
  the same commit (deltas 2, 3 and 4). Probe: the same `git grep` over `docs/`,
  which finds the three spellings delta 2 replaces.
- `TASK-QUEUE.md` — the two entries' terminal moves (delta 5).

No other component is touched. evidence-kit/SPEC.md §Layout and configuration is
cited by deltas 3 and 4, and neither changes the header grammar or the arm order
it parses.

## Retired spellings

- `got="$(git hash-object -- "$C/P")"` — the SPEC's per-entry spelling of `got`'s source, replaced by the batch (delta 2).
- `capture time inside the loop` — the `got != reread` reading, re-phrased for a batch capture (delta 2).
- `four separate times` — the stale pack count in §The consumer smoke (delta 4).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Profile re-read off the landing push.** The first `gates` run carrying
      delta 1 is timed the way the profile above was. The Windows manifest loop's
      total should fall from about 237 s to a few seconds. If it does not, the
      batch is not what the leg runs, and that red is build's to fix.
- [ ] **Instruction surfaces: instruction only** — the replacement text above
      carries no grounds that belong in a delta.
- [ ] **Merged with no information lost** — each addition re-phrases the text it
      refines, and the merged §The consumer smoke reads as one document.
- [ ] **Amendment deleted** — this file is removed on merge, and none remain for
      the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every retired spelling above is gone from the
      tracked tree, `check-amendment-retired-spelling` is green, and the docs
      mirror is regenerated.
- [ ] **Terminal moves** — delta 5's two moves land before the drain stage.
- [ ] **Gaps filed** — cross-component gaps found during the work are filed as
      debt tasks.
