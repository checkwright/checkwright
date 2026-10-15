# SPEC amendment: failed-turn-reading

The keyed verdict reads only what a vendor's feed reports, and a spend limit that is no window never reaches the feed. A vendor's credit wall is one: its turn exits non-zero on the vendor's error and leaves the snapshot as it stood. So the next read answers what the snapshot answered before the wall, `RESET-OK` on a dead window or `OK` on a fresh one, and the next dispatch meets the wall again. Both have happened here: three runs of one adapter each exited 1 on the vendor's out-of-credits error under a `RESET-OK` reading, and a later run did so under an `OK` reading of a 75-second-old snapshot.

The entry's Deliverable leaves the authoring stage one of two remedies. This amendment takes the second: a failed turn's vendor error reaches the snapshot through the consumer's producer. The first, a `RESET-OK` consequence saying an old reading predicts nothing of the new window, changes a word and no dispatch, and the recurrence was an `OK` reading it would not touch. Only the consumer can tell a credit wall from any other failed turn, since telling them apart reads the vendor's error and the kit parses no vendor output (§The keyed verdict). So the kit hands the producer the failed turn, and the producer classifies it.

## What changes

### (1) The producer reads a failed turn through `@FAILED_RUN@` {design-bearing} {user-facing: the foreign-budget-reset-ok-stale entry's Deliverable, its second remedy — a failed run's vendor error refreshing the snapshot through the consumer's producer}

`DELEGATION_KIT_FOREIGN_USAGE_CMD` gains a second token, `@FAILED_RUN@`. Where any word of an adapter's producer carries it, the executor runs that producer once more after a spawned turn of that adapter ends `FAILED`, with the token substituted by the turn's run directory.

- **When.** After a turn of `--foreign-run` or `--foreign-resume` whose adapter was spawned and whose line ends `FAILED` with an `exit=` of a non-zero status or `timeout`. Not after `OK` or `REFUSED`, and not after a `FAILED` whose `exit=` is `-` or `0`: a failure that precedes the spawn, a budget pause among them, or one after a clean exit, an unreadable sweep patch among them. None of those leaves a vendor's error to read. The call comes after the shape check and the patch, with the turn's `report.txt` and `stderr.txt` in place, and before the line prints.
- **The substitution.** On this call a word carrying `@FAILED_RUN@` has the run's directory substituted, absolute, `<GATE_SDK_TMP_DIR>/foreign/<key>/`. On the call before every keyed read it has the empty string substituted, so one producer serves both calls and tells them apart by that word. `@USAGE_FILE@` is substituted on both.
- **The terms.** The refresh's terms, unchanged: spawned directly with no shell from the toplevel, its output discarded, bounded by `DELEGATION_KIT_FOREIGN_TIMEOUT`, and fail-soft, so a non-zero exit, a spawn failure or the bound's expiry leaves the snapshot as it was and the line as it would have printed.
- **The line is unchanged.** Its `budget=` field stays the read before the spawn. The next read, a dispatcher's `--budget` or the next run's own, is where the producer's answer shows.
- **A producer without the token** is not called after a failed turn, so an adapter whose producer ignores failures pays no second spawn.
- **The crate.** `produce` in `native/src/hook/verdict.rs` takes the failed run's directory or none, and substitutes it under a `FAILED_TOKEN` constant beside `USAGE_TOKEN`. The two turn paths in `native/src/emit/foreign_run.rs`, the open and the resume, call it on a spawned turn ending `FAILED` where the adapter's producer argv carries the token.

Replacement for §The keyed verdict's bullet **The refresh is the adapter's own command.** **Not yet applied.**

> - **The refresh is the adapter's own command.** Where `DELEGATION_KIT_FOREIGN_USAGE_CMD` configures one, it runs before every keyed read, spawned directly with no shell from the toplevel, its output discarded. Where a word of it carries `@FAILED_RUN@` it also runs after every spawned turn that ends `FAILED`, a non-zero exit or the bound's expiry, with that turn's run directory in the word, and before a read with the word emptied. So a producer reads the vendor's error a failed turn left, which only the consumer can classify. Either call is fail-soft on §usage-verdict's terms: a non-zero exit, a spawn failure or the bound's expiry leaves the snapshot untouched. Its bound is `DELEGATION_KIT_FOREIGN_TIMEOUT`, the bound on everything the two arms spawn, so a consumer tunes one number and no second bound is minted. `DELEGATION_KIT_REFRESH_CMD` and `DELEGATION_KIT_REFRESH_MIN_AGE` are not read: the keyed read fires once per foreign command, never in a guard's burst.

Replacement for §Layout and configuration's `DELEGATION_KIT_FOREIGN_USAGE_CMD` row, its second sentence onward. **Not yet applied.**

> A word containing `@USAGE_FILE@` has that adapter's snapshot path substituted, absolute. A word containing `@FAILED_RUN@` has the run directory of a turn that just ended `FAILED` substituted, absolute, on the call that turn makes, and the empty string before a read. A producer carrying neither token is still run before every read (§The keyed verdict).

Replacement for the template line `# DELEGATION_KIT_FOREIGN_USAGE_CMD[] = <adapter>=@USAGE_FILE@` in `delegation-kit/templates/delegation-config.knobs`: the line stays and one line follows it. **Not yet applied.**

> `# DELEGATION_KIT_FOREIGN_USAGE_CMD[] = <adapter>=@FAILED_RUN@`

The release declaration (`.workflow/release-declarations.md`, §Behavior changes) gains one bullet: **delegation-kit `--foreign-run`, `--foreign-resume`, knob `DELEGATION_KIT_FOREIGN_USAGE_CMD`, delegation-kit/templates/delegation-config.knobs** — a producer word may carry `@FAILED_RUN@`: such a producer also runs after a spawned turn that ends `FAILED`, with that turn's run directory in the word, and before a read with the word empty, so it can write a spend limit the vendor reported only as a failed turn's error. Nothing to do; a producer without the token runs as before.

### (2) A producer writes a failed turn's wall and never moves a snapshot backwards {design-bearing} {user-facing: the foreign-budget-reset-ok-stale entry's Deliverable, its second remedy, as delta 1}

The producer obligations in §The keyed verdict's paragraph **The axes are read by length** gain the two a failed turn needs. A wall written from an error survives only if the next refresh, which reads the vendor's feed, leaves the newer reading in place: the feed's newest report can predate the wall, as it does wherever the failed turn added nothing to the feed.

Replacement for that paragraph. **Not yet applied.**

> **The axes are read by length.** A producer maps its vendor's shorter window onto the three mandatory lines and its longer window, where the vendor has one, onto the weekly pair. `updated_at` is the time the vendor reported the reading, never the time the producer ran: a producer that restamps an old reading turns budget-unknown into a false OK. A producer writes the snapshot atomically, omits what its source lacks, as the contract binds every producer, and leaves in place a snapshot newer than its own reading, since an older one would undo what a failed turn wrote. **A spend limit that is no window**, such as a credit balance, reaches the snapshot only from a failed turn. A producer finding its vendor's error for one in the turn's report or standard error writes it as an at-or-over reading of the short window. Its `updated_at` is the error's time, the turn's last write where the vendor stamps none, and its `resets_at` is the vendor's stated reset or, where the vendor states none, the time the consumer picks to try again.

Replacement for §The foreign-vendor run's honest limit **The oracle is as fresh as the consumer's producer.** **Not yet applied.**

> - **The oracle is as fresh as the consumer's producer.** Across a gap a by-product feed keeps a pause and loses an OK (§The keyed verdict). A reading whose short window died in that gap is `RESET-OK` only where its long window is under threshold, absent or itself reset. A spend limit the feed does not report costs the one turn that meets it, and then pauses only under a producer that reads failed turns. Once the retry time the producer wrote passes, the next run meets the limit again if it still stands.

### (3) The crate cases {mechanical}

The keyed-verdict rows in `native/src/emit/foreign_run.rs` gain, on a stub adapter exiting 1 and a stub producer that logs each call's argv and, when its failed-run word is non-empty and that directory's `stderr.txt` carries a marker, writes an at-or-over reading:

- the open's failed turn calls the producer with the run directory, and the next `--budget` read is `PAUSE` and the next open under a second key is `FAILED (budget: …)` with no run directory made;
- a resumed turn ending `FAILED` calls it with the session's run directory, and the next resume is paused with its turn counter held;
- the call before a read carries the empty word;
- no failed-turn call follows an `OK` turn, a `REFUSED` turn or a failure before the spawn, and none follows any turn of an adapter whose producer lacks the token;
- a timed-out turn calls it;
- a producer exiting non-zero on the failed-turn call leaves the snapshot read as it was.

`native/src/hook/verdict.rs` gains a unit row: `produce` substitutes the run directory for `@FAILED_RUN@` inside a word, and the empty string where none is given.

Replacement for §Testing's bullet **The producer.** **Not yet applied.**

> - **The producer.** A stub writing through `@USAGE_FILE@` feeds the read that follows it. A producer exiting non-zero, one outliving a one-second bound and one that cannot spawn each leave the snapshot read as it was. A producer carrying `@FAILED_RUN@` is called after a spawned turn ending `FAILED`, a timeout included, on an open and on a resume, with the run directory in the word, and before a read with the word empty. A wall it writes from the turn's standard error pauses the next read. No such call follows an `OK`, a `REFUSED` or a pre-spawn failure, nor any turn of an adapter whose producer lacks the token.

### (4) This repo's binding reads its vendor's credit error {design-bearing}

This repo's two adapters share one snapshot and one inline producer in the gitignored `scripts/delegation-config.local.knobs` (the entry's binding, operator direction 2026-10-07, lead-relayed, not a ruling). Each adapter's producer argv gains a final word `@FAILED_RUN@` and two behaviours:

- **The guard.** It writes nothing when the snapshot's `updated_at` is at or past the reading it found in the vendor's session store.
- **The classifier.** Given a non-empty run directory, it reads that directory's `stderr.txt` and `report.txt` for the vendor's error, `Your workspace is out of credits`. Finding it, it writes `five_hour_used_pct=100`, `updated_at` from the mtime of the file that carried it, and `five_hour_resets_at` one hour after `updated_at`, the retry time this repo picks, since the error states no reset. Finding none, it exits non-zero, leaving the snapshot as it was.

The overlay is untracked, so this delta lands no tracked change, and its landing commit's message names what the overlay now holds. Build runs the classifier against a scratch run directory holding the error, and the guard against a snapshot newer than the store's newest reading.

The error text is on record rather than re-run: the close and validate commits of 2026-10-06 and 2026-10-13 record the adapter exiting 1 with that text and an empty report, and no failed turn's run directory survives here (`.tmp/foreign` is absent). The classifier reads both files, so the delta rests on neither placement.

## Producers and consumers

- **The failed-turn call.** Producer: the open and resume paths of `native/src/emit/foreign_run.rs`, on a spawned turn ending `FAILED`, reached wherever a consumer's producer carries `@FAILED_RUN@`; this repo's overlay sets it (delta 4). Consumer: the consumer's producer, by argv, once per failed turn. The one field, the run directory, is read by the producer's classifier. The empty word before a read is read by the same producer as "no failed turn".
- **The snapshot a failed turn writes.** Producer: the consumer's producer. Consumer: the keyed verdict, by file read, at the next `--foreign-run`, `--foreign-resume` or `--budget`, and through the last of these the lead's dispatch read (lifecycle-kit/templates/lead.md, *Read the stage's executor before you dispatch it*), which on `PAUSE` dispatches the stage on the master harness. Each field has its reader there: the percentage is read by the short-axis compare, `resets_at` by the RESET-OK branch and the pause line's countdown, and `updated_at` by age-STALE and by the producer's own guard.
- **The roster-holding readers of the token set**, by `grep -rn 'USAGE_FILE@' native/src delegation-kit scripts` over the tree: `native/src/hook/verdict.rs` (the constant and its substitution, delta 1), `native/src/emit/foreign_run.rs` (a test binding, delta 3), `native/src/knobs/delegation_kit.rs` (a validation row binding the token, which reads no token roster and needs no change), `delegation-kit/templates/delegation-config.knobs` (delta 1), `delegation-kit/SPEC.md` (deltas 1, 2 and 3) and the overlay (delta 4).
- **Knobs.** None is minted. `DELEGATION_KIT_FOREIGN_USAGE_CMD` gains a token and keeps its validation, which reads no token. The two arms' declared rosters are unchanged.
- **The seam.** The call and the token are kit mechanism and name no vendor. The classifier and the retry time are this repo's consumer content, in its overlay.

## Existing sections updated

- `delegation-kit/SPEC.md` §The keyed verdict — the refresh bullet (delta 1) and the paragraph on the axes (delta 2).
- `delegation-kit/SPEC.md` §Layout and configuration — the `DELEGATION_KIT_FOREIGN_USAGE_CMD` row (delta 1).
- `delegation-kit/SPEC.md` §The foreign-vendor run — the freshness honest limit (delta 2).
- `delegation-kit/SPEC.md` §Testing — the producer bullet (delta 3).
- `delegation-kit/templates/delegation-config.knobs` — the token line (delta 1).
- `native/src/hook/verdict.rs` — `produce`, the token constant and its unit row (deltas 1 and 3).
- `native/src/emit/foreign_run.rs` — the failed-turn call in the open and resume paths, and the crate rows (deltas 1 and 3).
- `.workflow/release-declarations.md` — the §Behavior changes bullet (delta 1).
- `scripts/delegation-config.local.knobs` — the overlay's producers, untracked (delta 4).
- `docs/delegation-kit/SPEC.md` — generated, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment adds a token, a call and two producer obligations, and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `foreign-budget-reset-ok-stale` moves to Done with `--queue done` in the merging commit, ahead of the drain stage.
