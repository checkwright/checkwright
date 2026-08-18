# SPEC amendment: roadmap-port

**The last shell gate in queue-kit, and the one member of the freshness family whose port was held
on a pairing rather than on a criterion.** `check-roadmap-fresh` (111 shell lines) becomes a
`.gate`-declared registry member of the binary. It is `cohort-held-members-port-prerequisites`'
increment for this iteration and retires that entry's last recorded hold on this member —
gate-sdk/SPEC.md §The first cohort relabelled the hold off criterion 7 in 2026-08-12 and left it
standing on cohort composition alone, and the composition arrived at scope's 2026-08-18 cut.

**Nothing here re-argues the hold, the emitter-design ruling, or the cohort.** The emitter design
was answered and merged (a ported emitter is a non-gate arm); the emitter itself ports under
gate-sdk/SPEC-emitter-tail.md. What this amendment owes is the gate's own three assertions on the
new substrate, the descriptor the in-process call forces, and the atomicity that keeps queue-kit's
never-disagree guarantee true across the change.

## What changes

### 1. The gate becomes a descriptor and a crate module

`queue-kit/checks/check-roadmap-fresh.sh` is deleted and replaced by
`queue-kit/checks/check-roadmap-fresh.gate` plus `native/src/gates/roadmap_fresh.rs`. The
`scripts/gates.list` registration is unchanged — the member was always registered; only its
declaration spelling moves, which is what keeps criterion 1 satisfied through the port.
**design-bearing**

Every behavioral contract queue-kit/SPEC.md §check-roadmap-fresh states survives verbatim: the
three assertions, the fail-closed conditions, the empty-`QUEUE_KIT_ROADMAP_FILE` clean skip, and
the two-argument hermetic form. This amendment changes the substrate, not the invariant.

### 2. Assertion B runs before assertion A, and the ordering is carried across deliberately

The shell form runs field validation first, on the ground its own directive states: **a tag naming
an unconfigured horizon is silently dropped from the emission, so a freshness verdict taken before
the fields are validated would pass a page that quietly lost an item.** That ordering is a
correctness property of the pair of assertions, not an implementation accident, and it is named
here because a port that reads the three assertions as an unordered set would lose it with nothing
reddening. **design-bearing**

### 3. Assertion A calls the emitter in-process and reads the block through the shared marker module

`emit::roadmap::emit(&[])` replaces the `bash queue-kit/bin/roadmap.sh --emit` spawn, and
`marker::read_block` replaces the shell marker extraction — the same in-process shape
`check-footprint-fresh` and `check-value-rollup-fresh` already carry. A missing page, missing
markers, or an unbalanced marker pair stays fail-closed (exit 2): an absent projection under a
configured path is a broken install, not a clean skip. **design-bearing**

The report cap this family owed is **already discharged** — gate-sdk/SPEC.md §The first cohort
records the crate constant landing with its first readers at §The consumer remainder cohort, so
this member consumes it rather than re-deciding it.

### 4. Assertions B and C read the crate adapter, and that is what keeps the guarantee true

Both scan `TASK-QUEUE.md` through `native/src/queue.rs`'s `roadmap_entries` — the same function
the arm renders from. **design-bearing**

queue-kit/SPEC.md §lib/queue.sh excluded `queue_roadmap_entries` from the shell/Rust split *for the
opposite reason* from `queue_live_slugs`: its only two consumers are the emitter and this gate, so
while both sat on one shell adapter the two could not disagree about what an entry claims. After
this port both sit on one **crate** function, so the exclusion's reasoning is satisfied rather than
repealed. gate-sdk/SPEC.md §The first cohort's per-member row names the alternative precisely —
*porting the gate alone duplicates it with nothing machine-held* — and delta 6 is what forecloses
it.

### 5. The consumer's lane vocabulary crosses the config bridge; the crate carries no lane name

Assertion B validates every `[roadmap:]` tag's two fields against `QUEUE_KIT_HORIZONS` and
`QUEUE_KIT_TRACKS`, read with `walk::knob_array` off the `GATE_SDK_KNOB_*` bridge that
`gate_command` builds for a `.gate`-declared member. The descriptor declares both knobs; the crate
ships the `<horizon>/<track>` **grammar** — the slash split, the at-most-one-tag rule, the
membership test — and not one configured value. **design-bearing**

This is the provenance seam at its sharpest point in this iteration: horizons and tracks are this
repo's editorial posture (`scripts/queue-config.sh`), queue-kit/SPEC.md §The tag algebra already
rules that a kit literal spelling them *would ship one project's roadmap posture as everyone's*,
and a gate is where such a value would most naturally get baked in as a match arm. The bridge is
the mechanism that prevents it, and the descriptor's knob declaration is the only thing that makes
the bridge carry them.

### 6. This gate and `--emit-roadmap` land in one commit

`check-roadmap-fresh` spawns `queue-kit/bin/roadmap.sh`, which
gate-sdk/SPEC-emitter-tail.md deletes. Landing the gate first duplicates the adapter across two
substrates; landing the arm first leaves a shell gate spawning a deleted script. **Neither half is
independently shippable, and a batch cut that separates them yields a broken tree rather than a
schedule.** The constraint is stated on both amendments so neither can be read alone and get it
wrong. **design-bearing**

### 7. The descriptor carries the crate modules the gate reaches, not only its data corpus

Today's `# graph:` manifest is
`couples=TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh dir=one valve=none tier=precommit
trigger=TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh`, and `# install: on-surface`. The ported
descriptor keeps all of that and **adds every crate module the gate reaches transitively** —
`native/src/gates/roadmap_fresh.rs`, `native/src/emit/roadmap.rs`, `native/src/queue.rs` and
`native/src/marker.rs`, the last two shared with the emit side of the compare — per
gate-sdk/SPEC.md §The non-gate arm. It also declares `QUEUE_KIT_HORIZONS`, `QUEUE_KIT_TRACKS`,
`QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_ROADMAP_FILE` and `QUEUE_KIT_ROADMAP_MARKER` so the bridge
carries them (delta 5). **design-bearing**

Omitting a module leaves the gate registered and green while the hook's `staged_matches` trigger,
derived from `couples=`, never fires it on the edit that broke the page.

### 8. The fixture pair is kept, and is explicitly not the port's parity proof

`queue-kit/gate-tests/check-roadmap-fresh/{good,bad}/` keep driving the two-argument
`[projection-file] [emit-file]` form, which is the hermetic mode the pair exists for: a freshness
gate offering only its bare form has no fixture that does not read the live queue. **mechanical**

**What the pair proves is bounded, and gate-sdk/SPEC.md already attested the failure.** Both cases
pass pre-baked files that steer assertion A off the live emitter, so a ported pair *"would have
gone green over an arm with no implementation"* — the same vacuity the `check-gate-output`
disposition exists to close, arriving through a different door. The arm's proof is the
transition-scoped parity run in gate-sdk/SPEC-emitter-tail.md delta 10; the pair proves the
comparator and the two-argument plumbing, and is cited for exactly that.

### 9. `cohort-held-members-port-prerequisites` keeps its entry, minus this member's hold

The entry's deliverable is the corpus of held members, and this amendment delivers one increment
of it, so the terminal move is a **demotion** rather than a Done move — the branch
canon-kit/SPEC.md §Merging an amendment states explicitly, and the same one the entry took on
2026-08-16. `check-tree-terms` still owes its criterion-4 answer and the roster still owes the
`# port-until:` spelling, so a Done move would assert a finished deliverable that is not finished.
**mechanical**

## Producers and consumers

**New interface: `check-roadmap-fresh` as a binary subcommand.**

- *Producer* — `gates::REGISTRY` dispatch on the subcommand name, reached from the generated
  pre-commit hook and from `run-gates.sh`. Enabling config: the `GATE_SDK_KNOB_*` bridge
  `gate_command` emits for a `.gate`-declared member, from the five knobs delta 7's descriptor
  declares. This repo sets all five in `scripts/queue-config.sh` and gate-sdk's defaults, so the
  producer's config is live rather than test-only.
- *Consumer* — the battery, at every pre-commit staging `TASK-QUEUE.md`, `ROADMAP.md` or
  `scripts/queue-config.sh`, and the whole-tree `run-gates.sh` run as the backstop for a commit
  outside that staged view.

**New interface: the gate's in-process call into `emit::roadmap::emit`.**

- *Producer* — `roadmap_fresh.rs` at assertion A. *Consumer* — the byte-compare against the
  marker block; nothing else reads the returned string.
- *Named reader for the coupling this creates* — `check-graph`, which reads the descriptor's
  `couples=` at every battery run and regenerates the hook's trigger set from it. This is the field
  that would otherwise have no reader, and delta 7 is what gives it one.

**Interface changed, not added: `native/src/queue.rs::roadmap_entries`.** It is *introduced* by
gate-sdk/SPEC-emitter-tail.md delta 3; this amendment adds its second caller. Field readers, and
the transition at which each is read:

- tag count — read here only, at assertion C (a second `[roadmap:]` tag on one entry).
- raw `<horizon>/<track>` field — read here only, at assertion B (slash split and membership).
- slug — read by both callers: here at assertions B and C to name the offending entry, and by the
  arm at render.
- declaration count — read here only, at assertion C (none, or two, is a violation).
- summary — read by the arm at render only; assertion C reads its *count*, never its text, which
  is what keeps the whitelist a whitelist.

No field is added by this amendment and none is left unread.

**Point 5 — the narrowing this change makes.** One tracked shell script leaves the tree
(`queue-kit/checks/check-roadmap-fresh.sh`), and `queue-kit/lib/queue.sh` loses one function.

- `check-settings-paths` — reds on a literal `.sh` command token in `permissions.allow[]` that does
  not resolve. **Not monotone under a deletion.** Probed: `.claude/settings.json` carries no grant
  naming this check script, so this deletion strands none. The grants that *are* stranded belong to
  the emitter deletion and are gate-sdk/SPEC-emitter-tail.md delta 7's.
- `check-readme-roster` — reds on name-set parity failure between a kit README's gate-roster block
  and the kit's `checks/` basenames, **in both directions and over both `*.sh` and `*.gate`
  spellings**. Monotone here by construction: the basename is unchanged, only the extension moves,
  and the gate reads both spellings precisely so a ported member does not drop out of its roster.
  Cleared by inspection.
- `check-docs-cmd` — assertion A reds on an invoked repo-relative `.sh` path in the governed doc
  set that does not resolve. **Not monotone.** Any doc fencing `bash queue-kit/checks/check-roadmap-fresh.sh`
  strands; the oracle is the grep, run at build rather than a roster kept here.
- `check-graph` — reds on a manifest path naming nothing tracked, and on a hook that does not match
  its generator's emission. **Not monotone**: the declaration path moves, so the hook must be
  regenerated in the same commit.
- `check-gate-substrate-parity` — assertion B reds when the `.gate` descriptor set and the roster
  `--list` prints are not equal. **Not monotone in either direction**: adding the descriptor
  without the registry module reds, and adding the module without the descriptor reds. This is the
  gate that makes delta 1 atomic whether or not anyone remembers it is.
- `check-gate-binary-fresh` — reds when the committed binary's `--source-stamp` does not match the
  crate's hashed source. **Not monotone**: it goes red on the crate edit and stays red until
  `bash gate-sdk/bin/build-native.sh` runs, which is a separate obligation from a green `cargo
  test`.
- `check-shellcheck`, `check-comment-tier`, `check-gate-output`, `check-gate-fail-closed`,
  `check-assertion-strength` — each reds on a **violation found** in the files it scans, with no
  count floor and no coverage floor. Monotone; removing one script can only remove violations.
  `check-gate-fixture-coverage` is the one to name explicitly: it reds on a **gate with no fixture
  pair**, a coverage floor and therefore not monotone — the pair is kept (delta 8), so it stays
  green, and it would not have if the port had dropped the pair as a shell artifact.

## Existing sections updated

- **queue-kit/SPEC.md §check-roadmap-fresh** — the declaration line becomes
  `checks/check-roadmap-fresh.gate` (hermetic, `precommit`, binary-dispatched), assertion A's
  emitter reference becomes the in-process arm, and the assertion-B-first ordering (delta 2) lands
  as prose in the section rather than surviving only as a directive on a deleted script. Owned by
  deltas 1, 2, 3 and 5.
- **queue-kit/SPEC.md §lib/queue.sh** — its `queue_roadmap_entries` exclusion paragraph names *this
  gate* as one of the two consumers. gate-sdk/SPEC-emitter-tail.md delta 3 owns the paragraph's
  rewrite; this amendment owns the half of it that names the gate, and the boundary is stated so
  the sentence is not edited twice or missed by both.
- **queue-kit/README.md** — the gate-roster annotation for this member, whose declaration
  extension moves. Owned by delta 1.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** — the
  `check-roadmap-fresh` row of the per-member cost table, and the two-members-held block whose
  first bullet is this member's hold. The hold is now **spent**, not lifted retroactively: the
  record of why it stood stays, and what changes is that the sequence it waited for arrived.
  Owned by delta 9.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — re-derived rather than
  inherited, on the rule §The first budget batch established: a port moves a declaration path from
  `<kit>/checks/<name>.sh` to `<name>.gate`, which can move *other* members into or out of the
  substrate-sensitive set. Assertion C is re-run after the descriptor lands; the table gains a row
  only if it says so. Owned by delta 1.
- **TASK-QUEUE.md `cohort-held-members-port-prerequisites`** — the `check-roadmap-fresh` block
  becomes a spent hold at the demotion, leaving `check-tree-terms` and the `# port-until:`
  spelling as the entry's remaining work. Owned by delta 9.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for `checks/check-roadmap-fresh.sh` and the
      `queue_roadmap_entries` shell adapter; nothing dangles.
- [ ] **Landed atomically with `--emit-roadmap`** (delta 6), and the fixture pair kept but not
      cited as the arm's parity proof (delta 8).
- [ ] **The binary rebuilt** — `bash gate-sdk/bin/build-native.sh` run in the same commit; a green
      `cargo test` does not discharge it.
- [ ] **The entry demoted, not moved to Done** (delta 9).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
