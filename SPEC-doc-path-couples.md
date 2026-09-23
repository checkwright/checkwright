# SPEC amendment: doc-path-couples

gate-sdk/SPEC.md §The `# graph:` manifest rules that a kit-shipped descriptor names a consumer path through a token and never as a literal. A literal freezes one consumer's layout into every adopter's trigger set, so the hook never fires on the adopter's real file. The same section says what to do with a literal the rule does not keep. A file knob the member declares whose value is exactly that literal becomes `knob:<NAME>`. A directory knob or locator plus a remainder becomes `knob:<NAME>/<remainder>`. A literal neither reaches marks a member reading a consumer path that no knob configures, which is filed as a gap. This unit is that gap: sixteen literal spellings across fifteen kit members, and each one takes a disposition here — corrected at align from the filed "eighteen literals across thirteen kit members", which undercounted the members the five dispositions below actually name and whose literal tally did not reconcile under any counting convention (§Verified at align, below the dispositions).

**The ruling: the literals fall into five dispositions, and only two of them mint a name.**

1. **A hand-enumerated cover for a walk that already has one.** The four `check-action-*` gates walk every `*.yml`/`*.yaml` from the scan root (`--reads`: `.` `ext:lit:yml,yaml`). Their `couples=` lists `.github/` globs, `docs/_config.yml` and `kit:templates/` globs. §check-action-run-shell already says the manifest couples the *walked* surface. The cover that walk takes is `*.yml,*.yaml`, the extension cover §check-reads-couples sanctions for a kit-literal filter at any depth, with `check-action-job-ref` as the live instance. No knob is involved.
2. **A transcription of this repo's value beside a token that already expands it.** `docs/*.md` in `check-install-claim`, `check-payload-claim` and `check-prose-tells`. `TASK-QUEUE.md` and `*/SPEC.md` in `check-scratch-citation`. `.claude/agents/*.md` and `.claude/commands/*.md` in `check-surface-ratchet`. Each descriptor already carries the `knob:` token for the knob whose value this repo set to that literal, so the literal is a second, frozen spelling of consumer config. The "keep literals beside the token" clause does not bind here: it covers a kit *default* corpus that is a runtime alternative to the knob. None of these literals is on a default branch. `manifest_files()`' default reaches canonical specs, READMEs and the agent file, never `docs/*.md`, and the scratch-citation knob's derived default is the queue file.
3. **A literal equal to a declared file knob's default.** `CLAUDE.md` is `CONTEXT_KIT_SURFACES`' default, and both `check-footprint-fresh` and `check-surface-ratchet` read and declare that knob. `VISION.md` is `CANON_KIT_DUP_SURFACES`' default for `check-surface-duplication`. `.claude/commands/*.md` is this repo's value of `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`, which `check-close-surfaces` reads and declares and whose descriptor carries no token for it yet. Each becomes `knob:<NAME>`. That reverses canon-kit/SPEC.md §Layout and configuration's verdict that `CANON_KIT_DUP_SURFACES` is deliberately uncoupled. That verdict reasoned from §check-reads-couples' walk class, and a named-file read is outside that class. But the manifest's covering rule — `couples=` covers every path the gate reads — and its file-knob conversion are what bind a named-file read, and they bind it.
4. **A hard-coded consumer path in the code.** `check-footprint-fresh` and `check-enforcement-fresh` each default their projection to a crate constant (`DEFAULT_PROJECTION`, `docs/footprint.md` and `docs/enforcement.md`), and their help text names the same path. These two mint knobs: `CONTEXT_KIT_FOOTPRINT_FILE` and `GATE_SDK_ENFORCEMENT_FILE`, each a scalar defaulting to today's value, on the convention that a kit's defaults are this repo's layout. An explicit positional still wins, so the two-argument hermetic mode is unchanged.
5. **Not a consumer path at all.**
   - `check-graph`'s `SPEC-*.md` and `*/SPEC-*.md` cover its `name:lit:SPEC-*.md` walk. The amendment glob is canon-kit's fixed generic vocabulary (canon-kit/SPEC.md §Layout and configuration). The cover the rule sanctions for a kit-literal name filter at any depth is the one token `*SPEC-*.md`.
   - `check-producer-liveness`'s `.tmp/*.run` covers a directory the gate takes from **argv**. Here that is the stage-entry pre-flight's `-- .tmp`. That directory is gitignored, so no commit ever stages a `.run` for the trigger to see. The literal comes off, on the ground a `commit-msg` gate's argv file already takes: a corpus supplied at invocation is the invoker's, not the descriptor's. `knob:EVIDENCE_KIT_LOCK_FILE` stays for the no-argument path mode.

**Out of this unit, filed rather than widened into.** The `# projection:` field carries the same literals: `check-footprint-fresh`, `check-enforcement-fresh`, `check-graph` and `check-roadmap-fresh` all name consumer paths there. Its grammar refuses a `knob:` token (gate-sdk/SPEC.md §The install disposition). That is a second field with its own readers (`check-projection-roster`, `--projection-witness`), so it is filed to the gap inbox with this amendment's commit. After this unit the two projections this unit touches stay green: `knob:CONTEXT_KIT_FOOTPRINT_FILE` expands to `*docs/footprint.md`, which covers the literal projection at the default. An adopter relocating the page reds `check-projection-roster` assertion A until that gap lands.

Two more `couples=` literals sit outside the five dispositions and the authoring-time roster reached neither: `gate-sdk/checks/check-enforcement-fresh.gate`'s `.github/workflows/*.yml`, and `lifecycle-kit/templates/stages/*.md` in both `context-kit/checks/check-footprint-fresh.gate` and `check-surface-ratchet.gate`. Neither fits a disposition above cleanly. `check-enforcement-fresh`'s emitter does not walk a `*.yml`/`*.yaml`-filtered corpus the way the action gates do — it greps for a `# enforce: class=monitor` marker under `GATE_SDK_ENFORCE_SCAN_DIR` (default `.`, pruning `templates/` and `gate-tests/`) with no extension filter, so disposition 1's extension cover does not fit, and the literal narrows that walk to the one file (`.github/workflows/site-health.yml`) presently carrying a live marker rather than covering the walk's actual root — a real gap, not a miscategorization. Whether `lifecycle-kit/templates/stages/*.md` is a deliberate carve-out of the generic `kit:templates/*.md` exclusion (the stage-skill templates being load-triggered rather than inert skeletons) or itself a frozen literal needing a token is likewise undecided here. Both need a disposition this amendment's authoring did not do the work for, so both are filed to the gap inbox with this amendment's commit rather than widened into on a guess, the same ground the `# projection:` field took above.

**Verified at align (2026-09-23).** The filed "eighteen literals across thirteen kit members" does not reconcile against the five dispositions' own enumeration under any counting convention tried: the dispositions above name fifteen distinct `.gate` members (not thirteen — `check-surface-ratchet` and `check-footprint-fresh` each carry a literal under two different dispositions, which is where the undercount came from), and sixteen distinct literal spellings (not eighteen, whichever of "count every disposition-1 gate's four literals separately" — thirty-two instances — or "count each distinct spelling once" — sixteen — is intended; the sixteen-spelling reading is what the intro above now states, since a `couples=` roster reasons about spellings, not per-file repetitions of the same one).

**Measured at authoring (2026-09-23).** `git grep -n couples= -- '*/checks/*.gate'` reproduces the sixteen literal spellings across fifteen members enumerated above (corrected at align from the filed "eighteen ... thirteen"), plus the two gap-filed literals this unit does not disposition. `checkwright-gates --reads check-action-pinning` prints `. ext:lit:yml,yaml`. The two `DEFAULT_PROJECTION` constants sit at `native/src/gates/footprint_fresh.rs:7` and `native/src/gates/enforcement_fresh.rs:7`. The `CONTEXT_KIT_SURFACES`, `CANON_KIT_DUP_SURFACES` and `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS` rows default to `CLAUDE.md`, `VISION.md` and `*/SPEC.md`. `scripts/lifecycle-config.knobs` passes `-- .tmp` to `check-producer-liveness` at every stage entry. `check-reads-couples` is clean on the live tree.

## What changes

### (1) The action gates couple their walked extension {mechanical}

**Not yet applied.** In `gate-sdk/checks/check-action-pinning.gate`, `check-action-run-shell.gate`, `check-action-gh-repo.gate` and `check-action-permissions.gate`, `couples=` becomes `*.yml,*.yaml`. The `.github/`, `docs/_config.yml` and `kit:templates/` globs all fall inside that cover and are dropped.

### (2) Transcribed values come off beside their tokens {mechanical}

**Not yet applied.**

- `canon-kit/checks/check-install-claim.gate` and `check-payload-claim.gate` drop `docs/*.md`. `SECURITY.md` stays, being a GitHub-fixed path.
- `canon-kit/checks/check-prose-tells.gate` drops `docs/*.md`.
- `lifecycle-kit/checks/check-scratch-citation.gate` drops `TASK-QUEUE.md` and `*/SPEC.md`.
- `context-kit/checks/check-surface-ratchet.gate` drops `.claude/agents/*.md` and `.claude/commands/*.md`.

### (3) Default-valued literals become their knob's token {mechanical}

**Not yet applied.**

- `context-kit/checks/check-footprint-fresh.gate`: `CLAUDE.md` → `knob:CONTEXT_KIT_SURFACES`.
- `context-kit/checks/check-surface-ratchet.gate`: `CLAUDE.md` → `knob:CONTEXT_KIT_SURFACES`.
- `canon-kit/checks/check-surface-duplication.gate`: `VISION.md` → `knob:CANON_KIT_DUP_SURFACES`.
- `lifecycle-kit/checks/check-close-surfaces.gate`: `.claude/commands/*.md` → `knob:LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`.

In canon-kit/SPEC.md §Layout and configuration, the clause "**`CANON_KIT_DUP_SURFACES` is deliberately not among them**, and that is a verdict rather than an omission — its members are read as named files, which §check-reads-couples rules outside the walk class, so coupling it would trigger a gate on paths no walk of its reaches." becomes:

> `CANON_KIT_DUP_SURFACES` is not among them, because its members are read as named files rather than walked. It is coupled all the same, as `knob:CANON_KIT_DUP_SURFACES`, by gate-sdk/SPEC.md §The `# graph:` manifest's file-knob conversion: `couples=` covers every path a gate reads, and a named-file read is still a read.

### (4) The two projections take knobs {design-bearing}

**Not yet applied.**

- **The knob rows.** `native/src/knobs/context_kit.rs` gains `CONTEXT_KIT_FOOTPRINT_FILE` (scalar, default `docs/footprint.md`), and `native/src/knobs/gate_sdk.rs` gains `GATE_SDK_ENFORCEMENT_FILE` (scalar, default `docs/enforcement.md`).
- **The code.** `native/src/gates/footprint_fresh.rs` and `native/src/gates/enforcement_fresh.rs` drop `DEFAULT_PROJECTION`. Each reads its knob as the positional's default, and its help text names the knob's resolved value rather than a literal.
- **The registry.** Each member's row in `native/src/gates/mod.rs` declares its knob.
- **The descriptors.** `docs/footprint.md` → `knob:CONTEXT_KIT_FOOTPRINT_FILE` in `context-kit/checks/check-footprint-fresh.gate`, and `docs/enforcement.md` → `knob:GATE_SDK_ENFORCEMENT_FILE` in `gate-sdk/checks/check-enforcement-fresh.gate`. Their `# projection:` lines are unchanged (see the gap above).

In context-kit/SPEC.md §Layout and configuration's knob list, after the `CONTEXT_KIT_SURFACES` bullet, add:

> - `CONTEXT_KIT_FOOTPRINT_FILE` — the committed footprint page `check-footprint-fresh` compares against the emitter; default `docs/footprint.md`. An explicit positional still wins, which is the two-argument hermetic mode.

In context-kit/SPEC.md §check-footprint-fresh, "byte-compares the committed `docs/footprint.md` against the footprint emitter" becomes "byte-compares the committed footprint page (`CONTEXT_KIT_FOOTPRINT_FILE`) against the footprint emitter". In the layout block, the `check-footprint-fresh.gate` comment "docs/footprint.md byte-fresh" becomes "the footprint page byte-fresh".

In gate-sdk/SPEC.md §Layout and configuration's environment-override roster, after the `GATE_SDK_GRAPH_ARTIFACT` entry, add: "`GATE_SDK_ENFORCEMENT_FILE` (default `docs/enforcement.md`; the committed enforcement-map page §check-enforcement-fresh compares — an explicit positional still wins)". In §check-enforcement-fresh, "Invariant: `docs/enforcement.md` byte-matches the enforcement-map emitter" becomes "Invariant: the committed enforcement-map page (`GATE_SDK_ENFORCEMENT_FILE`) byte-matches the enforcement-map emitter".

### (5) The two non-consumer literals take their stated form {mechanical}

**Not yet applied.**

- `gate-sdk/checks/check-graph.gate`: `SPEC-*.md,*/SPEC-*.md` → `*SPEC-*.md`.
- `evidence-kit/checks/check-producer-liveness.gate`: drops `.tmp/*.run`.

In evidence-kit/SPEC.md §check-producer-liveness, after the invariant paragraph, add:

> The descriptor couples only `knob:EVIDENCE_KIT_LOCK_FILE`: set mode's directory is the invoker's argument. A consumer's scratch directory is gitignored, so no commit stages a record for a trigger to see.

## Producers and consumers

- **The two new knobs** (delta 4).
  - Producers: each kit's table. This repo inherits both defaults, which equal today's constants, so no verdict here moves.
  - Consumers: each member's positional default, its help line, and the descriptor token.
  - Roster-holding readers: `check-knob-default-coupling` reds a knob whose owning SPEC states no default, so delta 4's SPEC rows state them. `check-graph`'s admissibility loop reds a token naming an undeclared knob, so delta 4's registry rows declare them. `--emit knob-roster` is derived. Each `templates/*.knobs` is a comment-only pointer with no row.
- **Every edited descriptor** (deltas 1 to 5). Reader: the generated pre-commit hook, which `check-graph` assertion D holds fresh, so the hook and the graph artifact are regenerated in the landing commit.
- **Point 5.** Each delta narrows or re-spells a *trigger*, not a scanned corpus. The one reader whose verdict turns on a couples field's reach is `check-reads-couples`, which asks whether every walked tracked file is covered. Delta 1's extension cover contains every path the walk reads. Deltas 2 and 3 replace a literal with a token whose expansion contains it at this repo's binding. Delta 5's `*SPEC-*.md` contains both globs it replaces. `check-producer-liveness` declares no walk root (`--reads` prints nothing for it), so that gate asks nothing of it. `check-projection-roster` assertion A reds a projection glob the member's own `couples=` does not cover. The two projections stay covered by delta 4's tokens at the default, and no other touched member declares a projection.
- **Point 6.** The obliged corpus is the sixteen literal spellings across fifteen members, enumerated by the probe above (corrected at align — see §Verified at align). Each one's satisfying value is named in deltas 1 to 5. The two literals align's audit found outside that corpus are gap-filed rather than obliged here.

## Existing sections updated

Roster from `git grep -n couples= -- '*/checks/*.gate'`, `git grep -n "DUP_SURFACES\|docs/footprint.md\|docs/enforcement.md" -- '*/SPEC.md'` and `git grep -n "DEFAULT_PROJECTION" native/src`, run 2026-09-23.

- `canon-kit/SPEC.md` §Layout and configuration (delta 3).
- `context-kit/SPEC.md` §Layout and configuration and §check-footprint-fresh (delta 4).
- `gate-sdk/SPEC.md` §Layout and configuration and §check-enforcement-fresh (delta 4).
- `evidence-kit/SPEC.md` §check-producer-liveness (delta 5).
- The four `gate-sdk/checks/check-action-*.gate` descriptors (delta 1).
- `canon-kit/checks/check-install-claim.gate`, `check-payload-claim.gate` and `check-prose-tells.gate`, `lifecycle-kit/checks/check-scratch-citation.gate` and `context-kit/checks/check-surface-ratchet.gate` (delta 2).
- `context-kit/checks/check-footprint-fresh.gate`, `context-kit/checks/check-surface-ratchet.gate`, `canon-kit/checks/check-surface-duplication.gate` and `lifecycle-kit/checks/check-close-surfaces.gate` (delta 3).
- `native/src/knobs/context_kit.rs`, `native/src/knobs/gate_sdk.rs`, `native/src/gates/footprint_fresh.rs`, `native/src/gates/enforcement_fresh.rs`, `native/src/gates/mod.rs`, `context-kit/checks/check-footprint-fresh.gate` and `gate-sdk/checks/check-enforcement-fresh.gate` (delta 4).
- `gate-sdk/checks/check-graph.gate` and `evidence-kit/checks/check-producer-liveness.gate` (delta 5).
- `scripts/git-hooks/pre-commit` and the graph artifact, regenerated (all deltas).
- `.workflow/release-declarations.md`, one Behavior changes bullet (all deltas):
  - fifteen kit descriptors re-spell their triggers through knobs and covers, so an adopter's hook fires on the adopter's own configured files; regenerate the hook;
  - two new knobs, `CONTEXT_KIT_FOOTPRINT_FILE` and `GATE_SDK_ENFORCEMENT_FILE`, name the projections that were fixed paths.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`, `docs/context-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` and `docs/evidence-kit/SPEC.md`.

## Retired spellings

- None — the two `DEFAULT_PROJECTION` constants delta 4 removes share their name with three consumer-declared freshness members' constants (`scripts/check-install-evidence-fresh`, `check-trajectory-fresh`, `check-value-rollup-fresh`), which are this repo's own configuration and stay.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the two knobs and every re-spelled trigger.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `doc-path-hardcoded-reads` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** The `# projection:` literal class, and the two `couples=` literals align's audit found outside the five dispositions (`check-enforcement-fresh.gate`'s `.github/workflows/*.yml` and `lifecycle-kit/templates/stages/*.md` in `check-footprint-fresh.gate`/`check-surface-ratchet.gate`), are filed to the gap inbox with this amendment's commit.
