# SPEC amendment: doctor-shell-gate

**`doctor` owes `bash` to an adopter who registered a shell gate of their own.** Today `bash`'s `derived` audience (§bin/env-probe) is read over the kit roots and over the anchor's fence-run corpus. A gate the adopter writes from `gate-sdk/templates/check-skeleton.sh` sits in the gates dir, in no kit root. `toolfloor::owed`'s `registered` arm skips a name with no crate `REGISTRY` row. So a native-Windows adopter who registers a shell gate without Git for Windows' bash reads `DOCTOR: clean`, then meets the battery's exit 2 on the first run. The page states the requirement in prose and says `doctor` does not check it (docs/install.md §Writing your own shell gates). installer/SPEC.md §Requirements says the same: the derivation cannot see such a gate.

**Read at authoring:**

- `derived_audience_at` (`native/src/toolfloor.rs`) composes two arms: the kit-root arms (`kit_arms_at`) and the fence arm (`fence_executor`). Its callers are:
  - `installed_selection` in `native/src/installer/doctor.rs`, over the installed tree;
  - `init`, in `native/src/installer/init.rs`, over the payload;
  - `derived_audience_here`, for the env-probe arm and bare `doctor`;
  - its own unit tests.

  `check-install-toolchain` reads `derived_kit_audience_here`, the kit-root arms alone. The roster comes from `git grep -n 'derived_audience_at\|derived_audience_here\|derived_kit_audience_here' -- native/src`.
- `registry::resolve` tries each resolve dir in order, the gates dir first (`registry::resolve_dirs`). Within a dir `.sh` beats `.gate`, so a consumer `.sh` shadowing a shipped member also resolves to the gates dir.
- `ls scripts/*.sh` lists no `check-*.sh`, so this repo registers no consumer shell gate, and its own env-probe and `doctor` renderings are unchanged by the new arm.

## What changes

### (1) A fourth derivation arm: a registered member resolving to a shell declaration in the gates dir

{design-bearing} `derived_audience_at` takes the anchor's gates dir as a new argument; a caller with none passes nothing. It adds the **SDK root's kit name** (`kit_name(sdk_root)`, the floor-holder, named by the root it occupies and never by a literal) when two things hold:

- the anchor's registry (`registry::list_path(<anchor>/<gates dir>)`) lists a member whose resolution through `registry::resolve_dirs(<gates dir>, roots)` is a `.sh` file inside that gates dir;
- the SDK root is among the reader's roots, since the runner that spawns the gate is the SDK's.

A member resolving into a kit's `checks/` adds nothing here, because the kit-root arms already read that file's shebang. An unregistered `.sh` adds nothing either, since no battery runs it.

The callers pass the gates dir:

- `installed_selection` passes `crate::knobs::gates_dir()`, the dir its registry read already uses.
- `derived_audience_here` passes the same knob.
- `init` passes none: the payload holds no adopter content. The fence arm's honest limit covers this unchanged. A gate registered after `init` is owed from the next `doctor` or env-probe read, and until then the battery's fail-closed exit 2 on a missing `bash` keeps the gap loud.

The arm's predicate reads files the way the other arms do. An unreadable registry contributes nothing rather than failing the derivation, the fence arm's own degrade for an unreadable doc.

**Why the floor-holder's name, against §bin/env-probe's narrowing paragraph:** that narrowing keeps the SDK off the kit-root arms because every profile carries it, so naming it there would make `bash` unconditional. This arm is conditional on the anchor's content, as the fence arm is. And the kit whose runner spawns a shell gate is the SDK. Every selection carries the SDK, so the owed-predicate answers **owed** exactly when the arm fires. No fourth answer and no new audience value are minted.

### (2) Crate tests pin the arm

{mechanical} `native/src/toolfloor.rs` gains a unit test on the fence arm's scratch-anchor shape. The anchor carries a gates dir, a `gates.list` and a kit root `gate-sdk`, and the test asserts:

- a registered `check-x` with `<gates>/check-x.sh` derives `["gate-sdk"]`;
- the same member as `<gates>/check-x.gate` derives nothing;
- an unregistered `<gates>/check-y.sh` derives nothing;
- a registered member resolving only to `<kit>/checks/check-z.sh` derives nothing from this arm;
- with no gates dir passed, nothing is derived.

`owed` over a selection carrying `gate-sdk` with that derived list answers `Owed` for `bash`.

### (3) §bin/env-probe, installer §Requirements and the install page state the fourth arm

{mechanical} **Not yet applied.**

context-kit/SPEC.md §bin/env-probe:

- The third-arm paragraph's sentence "A **bash surface** is whatever any of the three arms reads." becomes:

  > **The fourth arm is read over the anchor's registry:** a member the anchor's `gates.list` registers, resolving to a `.sh` declaration in the gates dir, owes `bash` through the SDK root's kit, whose runner spawns it. It shares the third arm's honest limit, since `init` reads the payload and the adopter registers the gate later. A **bash surface** is whatever any of the four arms reads.
- The narrowing paragraph's opening, "**The floor-holding root is narrowed past**", gains after its first sentence: "That narrowing binds the two kit-root arms; the fourth arm names the floor-holder because it fires on the anchor's content, never on the selection."
- The `registered` bullet's last sentence becomes: "A registered name with no crate `REGISTRY` row is a consumer-declared shell gate and contributes nothing here, its command being the consumer's requirement (gate-sdk/SPEC.md §The program roster); the `bash` it runs under is owed through `derived`'s fourth arm."

installer/SPEC.md §Requirements, *The `bash` audience* bullet: the sentence "A shell gate an adopter writes is outside the audience, because it is not a kit file and the derivation cannot see it." and the one after it become:

> A shell gate an adopter writes and registers joins the audience through the derivation's registry arm, so `doctor` probes `bash` for it from the next read after registration. On native Windows Git for Windows' bash is the only shell a custom gate runs under.

docs/install.md §Writing your own shell gates: "`doctor` does not check that, because the roster owes `bash` only through the kits you vendor." becomes "`doctor` checks it once your `gates.list` registers the gate."

### (4) The site mirrors follow

{mechanical} `docs/context-kit/SPEC.md` and `docs/installer/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commit landing delta 3.

## Producers and consumers

- **The fourth arm.** The producer is `derived_audience_at`, at every `doctor` with an install, every bare `doctor`, and every `--emit env-probe`. The enabling configuration is an adopter's `gates.list` registering a member that the gates dir declares as a `.sh`, which is the documented custom-gate path (docs/install.md §Writing your own shell gates).

  Its consumers each read the returned kit list:
  - `toolfloor::owed`, through `Selection.derived`;
  - `doctor`'s toolchain block, which probes an owed member, sets the verdict on it, and renders the resolved audience `gate-sdk-only`;
  - the env-probe arm's rendered verdict, which marks the audience the same way.
- **Readers whose verdict moves, and their red conditions.** The arm only widens, so no reader reds on finding none.
  - `doctor` exits non-zero on an owed member that is absent or below its floor. For an adopter with a registered shell gate and no `bash`, it moves from clean to that refusal. This is the intended change.
  - The consumer smoke's profile loop (installer/SPEC.md §The consumer smoke) reads each profile's installed `doctor` report for a probed `bash` row. The scratch consumers it builds register no gates-dir `.sh` member, so no profile's subject set moves. The run was `git grep -n '>>.*gates.list\|GATES_DIR/check-[a-z-]*\.sh\|cat >.*GATES_DIR' -- installer/consumer-smoke/run-smoke.sh`, which finds no write of either, and its `gates.list` hits only read the file.
  - `every_audience_value_is_closed_over_the_kit_roots` resolves over the authoring tree, whose gates dir declares no `.sh` member, so its verdict is unchanged. Were one added, `gate-sdk` is a kit root and the test still holds.
  - `check-install-toolchain` reads the kit-root arms alone and is unchanged.
- **No new knob, arm, audience value or state.** The derivation takes one more argument, from a knob its callers already resolve.

## Existing sections updated

Roster produced by `git grep -n 'derived_audience\|bash surface\|outside the audience\|does not check that' -- '*.md' native/src`, with each hit read.

- `native/src/toolfloor.rs` — `derived_audience_at`, `derived_audience_here`, the new arm (delta 1) and its test (delta 2).
- `native/src/installer/doctor.rs` — `installed_selection`'s call (delta 1).
- `native/src/installer/init.rs` — its call, passing no gates dir (delta 1).
- `context-kit/SPEC.md` — §bin/env-probe: the third-arm paragraph, the narrowing paragraph, the `registered` bullet (delta 3).
- `installer/SPEC.md` — §Requirements, *The `bash` audience* (delta 3).
- `docs/install.md` — §Writing your own shell gates (delta 3).
- `docs/context-kit/SPEC.md`, `docs/installer/SPEC.md` — the regenerated mirrors (delta 4).

## Retired spellings

- None — the change adds a derivation arm and retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
