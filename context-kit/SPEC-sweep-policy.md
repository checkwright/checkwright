# SPEC amendment: sweep-policy

Pairs `session-sweep-horizon-baked` and `session-sweep-live-scratch`, which edit one function. The session-context hook's step 6 removes scratch on a constant age horizon (`STALE_SECS` in `native/src/hook/session_context.rs`, one day) and spares only `.gitkeep` and `.gitignore`. That breaks doctrine-kit/DOCTRINE.md Policy-as-choice, which asks for a calibration shipped as a consumer-selectable set with off among it. It also takes a session-lifetime artifact once its session idles past the horizon. Re-run at spec over a sandboxed scratch dir: an aged session-role marker and an aged directory, the member started over them with stdin from `/dev/null`, printed `Tidied 3 stale scratch path(s)` and removed both.

**Reach: context-kit, lifecycle-kit, installer, the crate and this repo's knob file.** Delta 3 rewrites one lifecycle-kit paragraph, so this amendment is cross-component and the audit stage follows spec.

**The keep-list, weighed against the icebox's `boundary-preserve-covers-names-not-lifetimes`.** That entry's objection is that a keep-list of names re-buys one omission per new session-lifetime artifact, where the property is a lifetime. The sweep is already a lifetime mechanism: its age guard approximates "no live session still reads this". A names list on top of it is the residue, and this amendment keeps the residue small two ways.

- The one artifact context-kit mints, the role marker, is spared as a kit invariant, not a keep-list default, on lifecycle-kit's ground for its lead journal: a defaulted array is replaced, not merged, when a consumer assigns it.
- Delta 3 states the lifetime choice where both reclaimers meet. A consumer whose scratch already dies at a boundary of its own turns the age sweep off, and that is a lifetime declaration, not a name.

The icebox entry's subject, the boundary wipe's own keep-list, is untouched.

**A survey witness never reads scratch.** The entry's second loss was "any path a survey record's witness names". A witness's `corpus` is git pathspecs diffed since a recorded revision, with `none` for an untracked input (lifecycle-kit/SPEC.md §The survey record), so the sweep cannot reach a corpus. What it can reach is an `oracle` command reading a scratch input, which a consumer keeping the sweep names on the keep-list.

## What changes

### (1) Step 6 takes its horizon from a knob and spares the role marker and a keep-list {design-bearing} {user-facing: session-sweep-horizon-baked's Deliverable, "the horizon as a context-kit knob with an off value", and session-sweep-live-scratch's, "the sweep sparing the role marker and a keep-list the consumer binds, on `LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s posture"}

Replacement for `context-kit/SPEC.md` §The session-context hook, step 6. **Not yet applied.**

> 6. **Scratch sweep** — reclaim `GATE_SDK_TMP_DIR` entries unmodified for longer than `CONTEXT_KIT_SWEEP_HORIZON_HOURS`, depth-first so a stale directory goes once it is empty. The line counts the paths removed. The age guard lets a concurrent same-checkout session's in-flight scratch survive. At `off` the step does nothing and prints nothing.
>    - **It never takes** `.gitkeep`, the scratch directory's self-ignore `.gitignore` (gate-sdk/SPEC.md §Layout and configuration), the marker `CONTEXT_KIT_SESSION_ROLE_FILE` names, or a `CONTEXT_KIT_SWEEP_PRESERVE` entry. The marker is a kit invariant rather than a keep-list default, since a consumer assigning the list would replace a default. Keeping it at any age costs nothing: a marker whose id has rotated matches no session.
>    - **The keep-list is anchored at the scratch root.** An entry names an immediate child, spared whole and never descended; an entry containing `/` spares nothing, and the step's line names it. A consumer whose scratch dies at a boundary of its own sets the horizon `off`; one keeping the sweep names what a long-lived or resumed session must still find past the horizon.

The marker is matched by path: the role-file knob's value and each swept path, both resolved against the toplevel, compared whole. A marker the consumer placed outside the scratch dir is never met, so the clause is inert there.

`native/src/hook/session_context.rs`:

- The `STALE_SECS` constant and its directive go. `Knobs` gains the horizon, read as `None` at `off` and otherwise as seconds, and the keep-list. `KNOBS` gains both names.
- `stale` takes the horizon. `sweep` returns no line at `None` without walking.
- `sweep_dir` takes the spare set. At the scratch root it skips a keep-list name and the marker's path. Below the root it skips only the marker's path and the two keep files it already skips.
- A keep-list entry containing `/` is named on its own line after the tidied count: `sweep keep-list entry '<entry>' spares nothing: CONTEXT_KIT_SWEEP_PRESERVE names scratch-root children`.
- The unit test `the_sweep_takes_only_stale_paths_and_spares_the_keep_files` gains: an aged marker survives; an aged keep-list directory survives whole, nested content included; a nested file whose basename equals a keep-list entry is taken; at `off` an aged path survives. Its directive is rewritten to name the marker and keep-list clauses.

### (2) context-kit's knob roster carries the two names {mechanical}

`context-kit/SPEC.md` §Layout and configuration, two bullets after `CONTEXT_KIT_SESSION_ROLE_FILE`'s. **Not yet applied.**

> - `CONTEXT_KIT_SWEEP_HORIZON_HOURS` — whole hours a scratch path stays unmodified before the session-context hook's sweep takes it (§The session-context hook, step 6): a positive integer, or `off`, which turns the sweep off; default `24`.
> - `CONTEXT_KIT_SWEEP_PRESERVE` — array of scratch-root names the sweep spares whole at any age; default empty. An entry containing `/` spares nothing. The session-role marker is spared without being named.

The same section's validator list gains a bullet, **not yet applied**: `a CONTEXT_KIT_SWEEP_HORIZON_HOURS that is neither a positive integer nor off;`. The hook reads every knob before its first line, so it meets a refused value as a knob that cannot resolve and declines. That is the existing path in §The session-context hook's first bullet, and nothing new is written for it.

`native/src/knobs/context_kit.rs`: `Row::scalar("CONTEXT_KIT_SWEEP_HORIZON_HOURS", "24")` and `Row::indexed("CONTEXT_KIT_SWEEP_PRESERVE", &[])`, and the `validate` rule beside the brevity cap's.

§The session-context hook's accepted limits lose the bullet "The marker ages out with the scratch sweep's day horizon, degrading to absent-signal behavior." Delta 1 makes it false. **Not yet applied.**

### (3) lifecycle-kit names where the two reclaimers meet {design-bearing}

Replacement for `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the paragraph opening **The scratch dir has a second reclaimer, and the two do not overlap.** **Not yet applied.**

> **The scratch dir has a second reclaimer, and the two do not overlap.** context-kit's session-context hook sweeps it at every session start, age-guarded so a concurrent same-checkout session's in-flight scratch survives (context-kit/SPEC.md §The session-context hook). The boundary wipe is deliberately **not** age-guarded. It fires once, at the iteration transition, where any scratch meant to cross is named in `LIFECYCLE_KIT_BOUNDARY_PRESERVE`, and an age guard there would leave the previous iteration's fresh residue behind. Neither reads the other, so they meet only in the consumer's config: once a stage journal `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` reads, or the lead journal the wipe spares, outlives the sweep's horizon, the sweep takes it. A consumer running both turns the sweep off, leaving scratch an iteration's lifetime, or keeps those journals on the sweep's keep-list.

### (4) This repo leaves scratch reclaim to the boundary wipe {mechanical}

`scripts/context-config.knobs` gains `CONTEXT_KIT_SWEEP_HORIZON_HOURS = off` under a `# comment-tier-exempt:` line giving the reason: the boundary wipe already reclaims `.tmp/` at every scope entry. With `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE = 1` set here, a stage journal idle past a day would refuse the next stage's entry. **Not yet applied.**

This repo sets no `CONTEXT_KIT_SWEEP_PRESERVE`. With the sweep off it has nothing to keep, and a list here would be a second spelling of the boundary keep-list for an inert reclaimer.

### (5) Two prose claims of a day-old sweep name the default {mechanical}

Delta 1 makes the horizon a knob, so two sentences that state the sweep's age as a fact hold only at the default. **Not yet applied.**

- `lifecycle-kit/SPEC.md` §The survey record, the sentence "whose age guard is shorter than an iteration" becomes "whose default age horizon is shorter than an iteration".
- `installer/SPEC.md` §The update notice, the cache paragraph's "the session-context hook's scratch sweep reclaims entries older than a day" becomes "the session-context hook's scratch sweep reclaims, at its default horizon, entries older than a day".

## Producers and consumers

- **`CONTEXT_KIT_SWEEP_HORIZON_HOURS` (deltas 1 and 2).**
  - Producer: the static defaults table at `24`, overridden by a consumer's knob file. This repo sets `off` (delta 4), so the default and the off value are each set by a deployed configuration: a fresh adopter runs the default.
  - Consumer: the session-context member, through `knobs()` at its start, once per hook fire. `stale` compares it, and `sweep` short-circuits on `off`. The table validator also reads it, refusing a malformed value for every context-kit reader.
  - Roster-holding readers: `KNOBS` in `native/src/hook/session_context.rs`, the member's declared-name list (§Layout and configuration, "a compiled member refusing a name its table does not declare") (delta 1); the defaults table (delta 2); `--emit knob-roster`, derived from that table. Probe: `git grep -n "CONTEXT_KIT_SESSION_ROLE_FILE" -- native context-kit scripts installer gate-sdk`, over the tracked tree, finds for the sibling knob the SPEC's two mentions, `KNOBS`, the `Knobs` read and the table row, and nothing else.
- **`CONTEXT_KIT_SWEEP_PRESERVE` (deltas 1 and 2).**
  - Producer: a consumer's knob file. The kit default is empty and this repo sets none (delta 4), so no deployed configuration sets it today. That is the point-1 exposure: the keep-list is policy-as-choice for a consumer keeping the sweep, and only the crate's unit test exercises it. Stated, not hidden.
  - Consumer: `sweep_dir` at the scratch root, per hook fire; the malformed-entry line's one reader is the session reading the brief.
  - Roster-holding readers: as for the horizon.
- **The role-marker exemption (delta 1).** Producer: `/lead`'s first step writing the marker (§The session-context hook, the session-role signal). Consumer: `sweep_dir`'s spare test. The identity match is unchanged: a spared stale marker matches no later payload id.
- **Point 5 — a narrowed corpus.** Delta 1 narrows what the sweep removes. Its readers are the brief's "Tidied" line, a count that only goes down, and no gate. The probe, `git grep -n "Tidied\|sweep_dir" -- ':!docs/posts'` over the tracked tree, returns the member itself and the queue entries this amendment pairs.
- **Point 6:** delta 4 obliges this repo's knob file alone.

## Existing sections updated

The roster came from `git grep -n -i "scratch sweep\|older than a day\|day horizon\|second reclaimer\|STALE_SECS\|sweep_dir" -- ':!docs/posts' ':!TASK-QUEUE.md'` and `git grep -n "CONTEXT_KIT_SESSION_ROLE_FILE"` over the tracked tree.

- `context-kit/SPEC.md` §The session-context hook, step 6 and the accepted-limits bullet (deltas 1 and 2).
- `context-kit/SPEC.md` §Layout and configuration, the knob bullets and the validator list (delta 2).
- `native/src/hook/session_context.rs`, the constant, `Knobs`, `KNOBS`, `stale`, `sweep_dir`, `sweep` and the sweep test (delta 1).
- `native/src/knobs/context_kit.rs`, the two rows and the validator rule (delta 2).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the second-reclaimer paragraph (delta 3), and §The survey record's "age guard" clause (delta 5).
- `installer/SPEC.md` §The update notice, the cache paragraph (delta 5).
- `scripts/context-config.knobs` (delta 4).
- `.workflow/release-declarations.md`, `## Knob changes`: one bullet naming the two context-kit knobs and their defaults, and one under `## Behavior changes` saying the sweep now spares the session-role marker at any age (deltas 1 and 2).
- `docs/context-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and `docs/installer/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- `STALE_SECS` — the constant gives way to the horizon knob (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Native build and suites** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-gate-tests context-kit/gate-tests` and `context-kit/smoke/install.sh` run green.
- [ ] **Done move** — both entries move to Done in the commit that merges this amendment, before the drain stage.
