# SPEC amendment: foreign-budget

The foreign-vendor run spends a window nothing reads: §The foreign-vendor run's honest limits record that no guard fires on the executor and that a foreign vendor's budget oracle is a later, vendor-keyed seam. This amendment builds that seam. It is the budget-oracle increment of the cross-vendor delegation entry, whose stage-contract increment stays design-pending.

**The shape, in one sentence:** each foreign adapter may name a usage snapshot in §The usage.txt contract and a consumer command that writes it, and the executor reads the account-keyed verdict's rule over that snapshot before it spawns.

**Why the kit parses no vendor feed.** A vendor's token-usage events are that vendor's schema, and a kit literal carrying one crosses the provenance seam (gate-sdk/SPEC.md §The provenance seam), the ground §The foreign-vendor run already gives for reading a report as bytes. Read at authoring off one installed vendor's on-disk session store, key names only and with no foreign run: its usage events carry a used percentage and a reset epoch for a shorter and a longer window beside cumulative token counts. Those map onto the snapshot's mandatory triple, its weekly pair and its token pair with no new key, so the existing wire contract is the whole seam and the mapping is the consumer's command.

## What changes

### (1) Two knobs bind an adapter to a snapshot and to the command that writes it {design-bearing} {user-facing: operator direction 2026-10-07 on the queue entry — the budget oracle, vendor-keyed beside the account-keyed verdict; the entry's seam ruling makes the oracle a consumer-config seam}

**Not yet applied.** Both are rows of delegation-kit's static table and bullets of §Layout and configuration.

- **`DELEGATION_KIT_FOREIGN_USAGE`** — indexed, each element `<adapter>=<path>`, at most one per adapter: the snapshot that adapter's window is read from, repo-root-relative or absolute. Default empty, which gives no adapter an oracle. Adapters spending one vendor account name one path, so the path is the key and no second vocabulary is minted. The table validator refuses an element with no `=`, an empty path, a second element for one adapter, and an adapter `DELEGATION_KIT_FOREIGN_ADAPTERS` does not configure.
- **`DELEGATION_KIT_FOREIGN_USAGE_CMD`** — indexed, each element `<adapter>=<word>`, the producer's argv being its words in element order, as `DELEGATION_KIT_FOREIGN_ADAPTERS`' are. A word containing `@USAGE_FILE@` has that adapter's snapshot path substituted. Default empty, which leaves the snapshot to whatever else writes it. The validator refuses a malformed element and an adapter with no `DELEGATION_KIT_FOREIGN_USAGE` element. It is a command seam: spawned directly with no shell, from the toplevel, so it takes no environment override.

**The snapshot is §The usage.txt contract's, unchanged, and the axes are read by length.** A producer maps its vendor's shorter window onto the three mandatory lines and its longer window, where the vendor has one, onto the weekly pair. `updated_at` is the time the vendor reported the reading, never the time the producer ran: a producer that restamps an old reading turns budget-unknown into a false OK. A producer writes the snapshot atomically and omits what its source lacks, as the contract already binds every producer.

**The kit ships no producer**, as it ships no adapter: each one reads a vendor's feed. The values usually live in the gitignored `.local` overlay beside the adapters.

**Sizing a consumer should know.** A vendor with no usage endpoint reports its window only as a by-product of a turn. Its snapshot is then as old as the last turn on that account, and a reading older than `DELEGATION_KIT_STALE_AGE` is budget-unknown (delta 2). The oracle therefore holds across a run of turns and lapses across a gap between them.

### (2) The keyed verdict is the account-keyed rule over an adapter's snapshot {design-bearing} {user-facing: operator direction 2026-10-07 on the queue entry — vendor-keyed oracles beside the account-keyed verdict}

**Not yet applied.** A new subsection of §The foreign-vendor run, citing §usage-verdict for everything it shares.

**One rule, a third caller.** The keyed verdict reads `DELEGATION_KIT_FOREIGN_USAGE`'s snapshot for one adapter through §usage-verdict's parse, its fail-closed readings and its check order, with the differences below and no others. Each difference is an input the account-keyed rule has and a foreign window does not, never a second calibration.

- **No identity arm.** The post-login reroute, its settle floor and the account-switch STALE read the master harness's credentials file and account config, which say nothing of another vendor's account. The keyed order is parse → RESET-OK → age-STALE → pause axes → OK.
- **No sample.** The keyed verdict appends nothing to `DELEGATION_KIT_USAGE_HISTORY`. That log's readers group by the master harness's account and read the newest sample as a roll witness, so a foreign sample would be read as the wrong window's boundary.
- **No `width=` field.** It reports the read-only `Agent` fan-out bound, which a foreign run is not counted under.
- **The refresh is the adapter's own command.** Where `DELEGATION_KIT_FOREIGN_USAGE_CMD` configures one, it runs before every keyed read, bounded, and fail-soft on §usage-verdict's terms: a non-zero exit, a spawn failure or the bound's expiry leaves the snapshot untouched and the read proceeds on it. `DELEGATION_KIT_REFRESH_CMD` and `DELEGATION_KIT_REFRESH_MIN_AGE` are not read: the keyed read fires once per foreign command, never in a guard's burst.

**Two thresholds, each defaulting to the account-keyed one.** `DELEGATION_KIT_FOREIGN_PAUSE_PCT`, derived default `${DELEGATION_KIT_PAUSE_PCT}`, and `DELEGATION_KIT_FOREIGN_PAUSE_PCT_LONG`, derived default `${DELEGATION_KIT_PAUSE_PCT_7D}`, both validated as the knobs they derive from are. A consumer who holds a foreign window only to spend it raises them; one who shares it with interactive use keeps headroom. `DELEGATION_KIT_STALE_AGE` is read as it stands.

**The line** is `foreign-budget: adapter=<adapter> used=<pct>% age=<n>s resets_in=<n>s -> <VERDICT> (<status>; <consequence>)`, the long window's percentage beside the short one's where the snapshot arms it. It honors §usage-verdict's verdict-string contract: reading, status and consequence on every line. A PAUSE names its axis as the *short window* or the *long window*, the long one when both fire.

**The verdicts and exits are §usage-verdict's, with one more non-reading.**

- **0** `OK` / `RESET-OK`.
- **1** `PAUSE`.
- **2** `STALE`, an unreadable or unparseable snapshot, and **`OFF`**: the adapter has no `DELEGATION_KIT_FOREIGN_USAGE` element. `OFF` says nothing gates this adapter and is not a fault. It takes the budget-unknown code because it is one, and its own word because its remedy is configuration rather than a refresh.

### (3) `--foreign-run <adapter> --budget` prints an adapter's keyed verdict and spawns no adapter {design-bearing} {user-facing: operator direction 2026-10-07 on the queue entry — the entry's attested demand is routing delegation to the vendor with headroom, which needs the reading before the dispatch}

**Not yet applied.** A second form of the existing arm, on `--foreign-resume <key> --close`'s precedent of a flag that selects a form: no arm-table row, no front-end case arm and no new spelling for a reader to learn.

- It takes exactly one operand, the adapter, runs that adapter's producer where one is configured, prints the keyed line on stdout and returns delta 2's exit. No clone is made and no adapter is spawned.
- An adapter the knobs do not configure, a second operand, or `--budget` beside `--mode` or `--key` is a shape refusal at exit 2 with the usage on standard error, as the arm's other malformed argv is.
- **Its reader is the dispatcher choosing an adapter.** The adapter is picked by name for the unit's class (§The foreign-vendor run), and headroom is the second input to that pick.

### (4) `--foreign-run` and `--foreign-resume` read the keyed verdict before the spawn, and a PAUSE spawns nothing {design-bearing} {user-facing: operator direction 2026-10-07 on the queue entry — the budget oracle; a verdict's exit 1 is its whole blocking signal (§usage-verdict), and the executor is the one chokepoint a foreign dispatch passes}

**Not yet applied.**

- **`PAUSE`** — nothing is cloned and nothing is spawned. The line is the arm's own, `exit=-` and `report=none`, ending `-> FAILED (budget: <the keyed line from its `used=` field on>)` at exit **2**. It is the `FAILED` class because a paused window is retried after it resets, the reading §Resuming a session already gives a turn a vendor's window refused. On a resume the turn counter does **not** advance, the previous turn's report is not rotated and the session stays open, as for every failure that precedes the spawn.
- **Every other verdict proceeds.** `STALE`, an unreadable snapshot and `OFF` are budget-unknown, and budget-unknown never blocks delegation (§usage-verdict).
- **The line gains `budget=<OK|RESET-OK|STALE|OFF|PAUSE>`**, after `key=<key>` on a run and after `turn=<n>` on a resume, on every line the arm prints past its argv check. Its reader is the dispatcher at the next dispatch: `STALE` or `OFF` beside an `OK` run says the turn was spent unbudgeted. A line printed before the adapter was resolved reads `budget=-`. `--close` prints no such field, since it spawns nothing.
- **The read precedes the clone**, so a paused run leaves no scratch behind and holds no key.

**Why the executor and not a hook.** The budget guard and the dispatch guard fire on the master harness's `Agent` tool, and a foreign run is a shell call. A hook on that call would have to parse a command line for an adapter name. The executor already holds the adapter, so the read costs it one in-process call, and no dispatch shape reaches a vendor without passing it.

**The declared knobs** of both arms gain `DELEGATION_KIT_FOREIGN_USAGE`, `DELEGATION_KIT_FOREIGN_USAGE_CMD`, the two thresholds and `DELEGATION_KIT_STALE_AGE`.

### (5) The run's honest limits and the kit's reader surfaces say what is now budgeted {mechanical}

**Not yet applied.**

- §The foreign-vendor run, **No guard fires**: the bullet keeps its first half, that neither `Agent`-tool guard fires and that gating a foreign run behind the master harness's window is refused, and its closing sentence is replaced by a pointer to the keyed verdict. Two limits join it. An adapter with no snapshot is unbudgeted and its line says `OFF`. And the oracle is as fresh as the consumer's producer: a by-product feed lapses to budget-unknown across a gap (delta 1).
- §The usage.txt contract: one sentence that the contract also serves a foreign adapter's snapshot, with the axes read by length (delta 1).
- `templates/agent-execution.md`, the **A foreign-vendor run is mechanical work returned through files** bullet: read an adapter's budget with `--budget` before choosing it, and read a `FAILED (budget: …)` line as a window to wait out, never as a failure to retry at once.
- `README.md`: the `--budget` form beside the two commands it already lists; `templates/delegation-config.knobs`: the four knobs, commented.

## Producers and consumers

- **The snapshot.** *Producer:* the consumer's `DELEGATION_KIT_FOREIGN_USAGE_CMD` argv, spawned by the keyed read. *Enabling config:* this repo binds both knobs in its gitignored overlay beside its adapters, written at build and exercised there against the vendor's on-disk session store with no foreign run; the crate's cases bind a stub producer. *Consumer:* the keyed verdict, through §usage-verdict's snapshot parse.
- **The keyed verdict.** *Producers:* the `--budget` form and the pre-spawn read. *Consumers:* the dispatcher, on stdout and the exit; and the executor, in process, grading exit 1 as its refusal, the grading `agent-budget-guard` makes of the account-keyed rule.
- **Every field of the keyed line has a reader**, the dispatcher at the adapter pick: `adapter` says whose window, `used` and `resets_in` say how much and until when, `age` says how far to trust it, and the verdict and its clauses carry the status and the consequence.
- **`budget=`** — read by the dispatcher at the next dispatch (delta 4). *Roster-holding readers of the verdict line's shape,* by `git grep -n 'report=none patch=none' native/src`: the arm's own crate cases, which assert the adjacency of `exit=`, `report=` and `patch=`. The field lands before `exit=`, so no existing assertion's substring is broken.
- **`OFF`** — read by the dispatcher, and by the executor as a proceed.
- **Roster-holding readers of a new knob,** by `git grep -l DELEGATION_KIT_FOREIGN_TIMEOUT`: delegation-kit's static table and its validator, each arm's declared-knob roster, §Layout and configuration, the generated `docs/` mirror and `.workflow/release-declarations.md`. `--emit knob-roster` derives from the table.
- **Roster-holding readers of a new command seam,** by `git grep -n FOREIGN_ADAPTERS gate-sdk/SPEC.md`: gate-sdk/SPEC.md's two spawn rosters, the network spawners and the consumer-changeable spawns, each of which names what `--foreign-run` and `--foreign-resume` spawn.
- **No corpus is narrowed and no enumerable corpus is obliged member by member**, so causal-completeness points 5 and 6 bind nothing here.

**Inferred, cannot run before build:** the account-keyed rule's parse and check order can be reached without its identity arms and its sample append through one parameterized function rather than a second copy of the rule — the split is made in `native/src/hook/verdict.rs`, and whether it is clean is read off that function once it is cut.

## Existing sections updated

- `delegation-kit/SPEC.md` §The foreign-vendor run — the keyed-verdict subsection (delta 2), the `--budget` form and its argv refusals (delta 3), the pre-spawn read, the `budget=` field on both verdict lines and both arms' declared knobs (delta 4), the honest limits (delta 5); §Resuming a session — the resume line's field and the paused turn's disposition (delta 4).
- `delegation-kit/SPEC.md` §Layout and configuration — four knob bullets (deltas 1 and 2); §The usage.txt contract — the keyed snapshot sentence (delta 5); §Testing — the cases below (deltas 2, 3 and 4).
- `native/src/knobs/delegation_kit.rs` — four rows and the validator's refusals (deltas 1 and 2).
- `native/src/hook/verdict.rs` — the rule reachable without its identity arms and sample (delta 2); `native/src/emit/foreign_run.rs` — the form, the read, the field and the knob roster (deltas 3 and 4).
- The crate's cases — a stub producer and a stub adapter: a PAUSE that spawns nothing and leaves no clone, a STALE and an OFF that proceed, a producer whose failure leaves the snapshot read as it was, a paused resume whose turn counter holds, and the `--budget` form's line and three exits (deltas 2, 3 and 4).
- `gate-sdk/SPEC.md` — the two spawn rosters name `DELEGATION_KIT_FOREIGN_USAGE_CMD` beside what the two arms already spawn (delta 1).
- `delegation-kit/templates/agent-execution.md`, `delegation-kit/README.md`, `delegation-kit/templates/delegation-config.knobs` (delta 5).
- `.workflow/release-declarations.md` — a row for the four knobs, the `--budget` form and the new `FAILED (budget: …)` outcome (deltas 1, 2, 3 and 4).
- `docs/delegation-kit/`, `docs/gate-sdk/SPEC.md` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).
- `TASK-QUEUE.md`, the cross-vendor delegation entry — its remaining-work list and landed-slices paragraph, rewritten inside the entry cap in the demoting commit (all deltas).

## Retired spellings

- None — no delta of this amendment retires a name; delta 5 rewrites one honest-limit sentence in place.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **No foreign run is owed.** Acceptance is the crate's stub-driven cases and this repo's producer read against the vendor's on-disk store. A live turn refreshing the feed is unobserved, and is filed as a gap rather than run without a grant.
- [ ] **The marker discharged** — the cannot-run claim under Producers and consumers is read off the function once it is cut.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Queue entry demoted, not done** — the entry outlives this amendment, its stage-contract increment being unbuilt: `--queue demote heterogeneous-agent-delegation`, in the merge commit and before the stage that drains the queue, its body brought inside the entry cap in that commit.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
