# SPEC amendment: roadmap-lag

**The closing stage reads which roadmap entries lag the work landing under them, and the mechanical half of that read is a queue-kit arm.** Close's roadmap read compares the roadmap arm's output at the iteration's start with its output now, and checks the first configured horizon for vacancy (lifecycle-kit/templates/stages/close.md, step 5). It catches a slug leaving the projection and an empty first horizon. It misses an entry the iteration worked toward while its tag stayed outside the first horizon, and an untagged entry whose slices keep landing, because neither changes the projection. Each instance was found by a session happening to read the entry, and the lag lasted until then.

The rule spans a queue-kit arm and lifecycle-kit's close template, so this amendment sits at the repository root.

**Read at authoring:**

- An authorized split writes a bidirectional citation: the parent links the child and the child links the parent (queue-kit/SPEC.md §check-queue-entry-budget, the split paragraphs), and `--queue split` refuses a split without both (§The queue verbs). The done move takes the child's body and rewrites the parent's link to it as a retired citation (§The icebox tier). So at close the child side survives only in history, and the parent side as a backticked slug.
- The splits this queue carries follow that shape: `lifecycle-queue-value-audit` links `gate-customer-value-audit`, which links it back; `queue-kit-gate-brevity` and `spec-brevity-residue` likewise.
- `walk_history` (`native/src/queue.rs`) is the one history walk, newest first, read by the entry-history and queue-history arms (queue-kit/SPEC.md §The shared queue adapters). The newest revision in which a done slug heads an entry is its last live revision.
- Close's step 5 runs the roadmap read after the moot sweep and before `--queue clear-done`, and close clears the done section every iteration, so at that point the done section holds the iteration's exits.
- The roadmap arm exits 2 with no configured horizon, and close skips its read then (`native/src/emit/roadmap.rs`, `emit`). "First configured horizon" already means the first `QUEUE_KIT_HORIZONS` member in close's vacancy read.
- The entry's inferred marker was run. A retroactive pass over today's pool finds no tagged entry lagging: the four entries outside the first horizon here, `gate-authoring-sdk-surface`, `companion-install-tier`, `benchmark-ab-experiment` and `hosted-attestation-service`, carry no landed slice. So no one-off pass is owed, and the arm starts from the pool as it stands.

## What changes

### (1) queue-kit ships the roadmap-lag arm

The binary gains `--emit-roadmap-lag`, reached as `run-gates.sh --emit roadmap-lag [<queue-file>]`, in a new module `native/src/emit/roadmap_lag.rs` with its arm-table row in `native/src/emit/mod.rs`. {design-bearing} {user-facing: the entry's deliverable, the lag read's mechanical half on an arm} **Not yet applied.** The row declares `QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DEFERRED_SECTION`, `QUEUE_KIT_ICEBOX_SECTION`, `QUEUE_KIT_DONE_SECTION` and `QUEUE_KIT_HORIZONS`. The arm reads the queue file and its history through `walk_history`, writes stdout only, and mutates nothing. Its contract is delta 2's section, and its unit tests cover the slice rule and the row rule over in-memory texts. `queue-kit/gate-tests/roadmap-lag.test.sh` builds a sandbox history in which:

- a split child lands under a parent tagged with a later horizon, which prints a row;
- one lands under an untagged parent, which prints a row with `-`;
- one lands under a first-horizon parent, which prints no row;
- two live entries link each other with neither done, which prints no row;
- the queue file sits outside a git work tree, which prints no row and a stderr line.

The test is listed beside the other bespoke queue-kit tests in §Layout and configuration (delta 2).

### (2) queue-kit/SPEC.md states the arm

queue-kit/SPEC.md gains a section after §check-roadmap-fresh. {design-bearing} **Not yet applied.**

> ### The roadmap-lag arm
>
> Which roadmap entries trail the work landing under them: a **non-gate arm of the binary** (gate-sdk/SPEC.md §The non-gate arm), `run-gates.sh --emit roadmap-lag [<queue-file>]`, registered under `--emit-roadmap-lag` and taking §The queue-index arm's dispositions. It reads the queue and its history, writes stdout only, and mutates nothing.
>
> - **A slice** of an entry P is a slug X in the done section whose entry links P at the newest revision in which X headed one, and which P's body still cites. That is the bidirectional citation an authorized split writes (§check-queue-entry-budget), read on the child's side through the history walk because the done move took the child's body (§The shared queue adapters).
> - **A row** is a top-level entry outside the icebox with at least one slice, whose `[roadmap:]` horizon is not the first `QUEUE_KIT_HORIZONS` member or which carries no `[roadmap:]` tag. It prints as the slug, a tab, the horizon or `-`, a tab, and its slices comma-joined, in queue order. No row means nothing lags.
> - **Exit** 0 with the report, and 2 on a usage error, an unreadable queue file, or no configured horizon, as §The roadmap arm exits. There is no 1: the arm reports and rules nothing.
> - **Degradations** are the retired set's (§The queue-edges arm). With no git work tree or no `git`, no slice can be confirmed, so the arm prints no row and says why on stderr. A shallow clone under-claims.
>
> **Its reader is the closing stage**, after the moot sweep and before the done section is cleared, which is when the done section holds the iteration's exits (lifecycle-kit/templates/stages/close.md, step 5). The arm reports and the session rules: it re-tags where an entry's recorded horizon condition settles the move and routes the rest with the motion read's findings. **A gate is refused**: a lagging horizon is a true state of the queue until the party who sets direction moves it, the ground §The roadmap arm gives for not gating a horizon's size.
>
> **Why a separate arm.** The roadmap arm's output is the page's body, which `check-roadmap-fresh` byte-compares. A lag report is a second grammar over a second input, the done section and the history walk, so it stands beside that arm and not inside it (§The queue-counts arm's refusal).
>
> **Honest limits.** A slice its parent records only in prose has no citation pair, and a reader finds it. Two entries citing each other for another reason read as a split, and so does a split child the moot sweep retired rather than a landing. The row names the slice, so the reading session discards either case at a glance.

Two passages elsewhere in queue-kit/SPEC.md take the arm:

- §Layout and configuration, first paragraph: "Neither are the `entry-history` arm (§check-queue-entry-budget), the `queue-history` arm (§The queue-history arm) and the `--queue` verbs (§The queue verbs)." becomes "Neither are the `entry-history` arm (§check-queue-entry-budget), the `queue-history` arm (§The queue-history arm), the roadmap-lag arm (§The roadmap-lag arm) and the `--queue` verbs (§The queue verbs).", and its bespoke-test list gains "`gate-tests/roadmap-lag.test.sh` for the roadmap-lag arm".
- §The shared queue adapters, the history-walk paragraph: "so the two arms read history one way" becomes "and the roadmap-lag arm reads a done slug's last live entry off it (§The roadmap-lag arm), so the three arms read history one way".

The comment directive on `walk_history` in `native/src/queue.rs` names the third reader in the same commit.

### (3) Close's roadmap read runs the arm

In lifecycle-kit/templates/stages/close.md, step 5, the sentence "Then read each `[roadmap:]` entry's recorded horizon condition, and re-tag one the range met in this stage's queue commit, regenerating the projection and naming the condition and the landing that met it." becomes: {mechanical} {user-facing: the entry's deliverable, the lag read in close step 5} **Not yet applied.**

> Then run `--emit roadmap-lag` over the queue file (queue-kit/SPEC.md §The roadmap-lag arm). Each row is an entry outside the first configured horizon, or untagged, under which a split slice landed this iteration. Read each `[roadmap:]` entry's recorded horizon condition, the arm's rows first, and re-tag one the range met in this stage's queue commit, regenerating the projection and naming the condition and the landing that met it. A row no recorded condition settles is one more finding of the motion read, routed with its others.

The sentences around it stand. `consult-intake-narrowing`, promoted this iteration, rewrites how the motion read routes its findings and the clause "a consultation sets new direction". This delta's sentence names no route, so its act is the same whether that unit lands in the same build batch, an earlier one or a later one.

### (4) lifecycle-kit/SPEC.md carries the grounds

In lifecycle-kit/SPEC.md §templates/stages/, *The close template*, the paragraph **A settled horizon is re-tagged, not filed.** keeps its first two sentences, and its last, the sentence opening **Honest limit:**, becomes: {mechanical} **Not yet applied.**

> **The lag read covers what the motion read cannot see.** The motion read reports a slug leaving the projection and a vacant first horizon, and an entry left outside the first horizon while its work lands changes neither. A landed split slice is the mechanical half of that lag, so an arm reports it and the session judges the rest (queue-kit/SPEC.md §The roadmap-lag arm). **Honest limit:** a slice recorded only in its parent's prose, and a met condition of any other shape, are found by reading, and nothing gates that any of the three reads ran.

### (5) The README and the site mirrors follow

`queue-kit/README.md` gains a usage line after the `--emit roadmap --write` line, `"$gates" --emit roadmap-lag          # entries outside the first horizon, or untagged, with a split slice in the done section`. {mechanical} {user-facing: the entry's deliverable, the arm's usage line} **Not yet applied.** `docs/queue-kit/SPEC.md`, `docs/queue-kit/README.md` and `docs/lifecycle-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commits landing deltas 2, 4 and this README line. `docs/footprint.md` and `docs/value.md`'s rollup block are regenerated (`--emit footprint > docs/footprint.md`, `--emit value-rollup --write`) in delta 3's commit, whose template markdown the footprint measures. That commit appends the unit's Behavior-changes bullet to `.workflow/release-declarations.md`, a new arm and a new close step a vendoring consumer meets (lifecycle-kit/templates/stages/build.md, *Declare what a vendoring consumer will meet*).

## Producers and consumers

- **The arm (delta 1).** Producer: a closing session running `--emit roadmap-lag` at step 5. The enabling config is a non-empty `QUEUE_KIT_HORIZONS`, which this repo sets (`scripts/queue-config.knobs`). Without it the arm exits 2 and close skips the read, as it skips the motion read.
- **The report's fields.** Each has one reader, the closing session at step 5:
  - the slug, read to re-tag the entry or route it;
  - the horizon, read against the entry's recorded condition, `-` meaning the curation question rather than a move;
  - the slices, read to name the landing in the queue commit and to discard a pair that is no split.
- **Readers whose verdict moves.** None. The arm is no gate, and the close step gains a command and no refusal. `check-roadmap-fresh` reads the projection, which only a re-tag the session makes changes, regenerated in that commit as step 5 already requires.
- **New name, roster-holding readers.**
  - The arm table in `native/src/emit/mod.rs` gains the row (delta 1).
  - `--emit knob-roster` and `check-reads-couples` read declared knobs off the table, so they need no edit.
  - The arm declares no walk root. The history walk is `git log` and blob reads, outside the analyzed class (gate-sdk/SPEC.md §check-reads-couples).
  - `check-comment-tier` holds the module's `spec:` directives to a resolvable section, which delta 2 mints.
- **Narrowing:** none.

## Existing sections updated

Roster produced by `git grep -n "emit roadmap\|walk_history\|queue-history\` arm\|re-tagged, not filed" -- ':!TASK-QUEUE.md' ':!docs/posts'`, with each hit read.

- `native/src/emit/roadmap_lag.rs` — new (delta 1).
- `native/src/emit/mod.rs` — the arm-table row (delta 1).
- `queue-kit/gate-tests/roadmap-lag.test.sh` — new (delta 1).
- `native/src/queue.rs` — the comment directive on `walk_history` (delta 2).
- `queue-kit/SPEC.md` — the new §The roadmap-lag arm, §Layout and configuration's first paragraph, and §The shared queue adapters' history-walk paragraph (delta 2).
- `lifecycle-kit/templates/stages/close.md` — step 5's re-tag sentence (delta 3).
- `lifecycle-kit/SPEC.md` — §templates/stages/, *The close template*, the settled-horizon paragraph (delta 4).
- `queue-kit/README.md` — the usage line (delta 5).
- `docs/queue-kit/SPEC.md`, `docs/queue-kit/README.md`, `docs/lifecycle-kit/SPEC.md` — the regenerated mirrors (deltas 2, 4 and 5).
- `docs/footprint.md`, `docs/value.md` — the regenerated footprint and rollup block (delta 5).
- `.workflow/release-declarations.md` — the unit's Behavior-changes bullet (delta 5).

## Retired spellings

- `so the two arms read history one way` — the history-walk paragraph's two-reader count, retired by delta 2.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `roadmap-horizon-lag-detector` moves to Done with `--queue done` in the commit deleting this file, at its build batch, before the drain stage.
